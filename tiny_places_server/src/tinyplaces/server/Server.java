package tinyplaces.server;

import java.io.IOException;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.SelectionKey;
import java.nio.channels.Selector;
import java.nio.channels.ServerSocketChannel;
import java.nio.channels.SocketChannel;
import java.nio.channels.spi.SelectorProvider;
import java.util.*;

/**
 * Main server class for Tiny Places
 */
public class Server implements Runnable
{
    private final InetAddress host;
    private final int port;
    
    private ServerSocketChannel serverChannel;
    
    private final Selector selector;
    
    private final ByteBuffer readBuffer = ByteBuffer.allocate(8192);
    private final ServerWorker worker;
    
    private final List <ChangeRequest> pendingChanges = new ArrayList();
    
    // Each client needs their own data list
    private final Map <SocketChannel, List> pendingData = new HashMap();

    /**
     * Instantiate a new server thread. We need only one.
     * 
     * @param host Server host
     * @param port Server port
     * @param worker The worker thread to do the actual work
     * 
     * @throws IOException In case the socket channel cannot be initialized
     */
    public Server(InetAddress host, int port, ServerWorker worker) throws IOException
    {
        this.host = host;
        this.port = port;
        this.selector = initSelector();
        this.worker = worker;
    }

    
    public void send(SocketChannel socket, byte[] data)
    {
        synchronized (this.pendingChanges)
        {
            // Tell the channel there is data to be written
            this.pendingChanges.add(new ChangeRequest(socket, ChangeRequest.CHANGE_OPS, SelectionKey.OP_WRITE));

            // Then queue the data to be written
            synchronized(this.pendingData)
            {
                List queue = this.pendingData.get(socket);
                if(queue == null)
                {
                    queue = new ArrayList();
                    this.pendingData.put(socket, queue);
                }
                
                // What is a good buffer size here?
                ByteBuffer dataBuffer = ByteBuffer.allocate(1 << 16);

                dataBuffer.put(data);
                dataBuffer.flip();
                
                queue.add(dataBuffer);
            }
        }

        // Wake up the selecting thread so it can make the required changes
        this.selector.wakeup();
    }

    
    @Override
    public void run()
    {
        while(true)
        {
            try
            {
                System.out.println("Processing " + pendingChanges.size() + " pending changes.");
                
                // Process any pending changes, lock the queue against changes
                synchronized(pendingChanges)
                {
                    for(ChangeRequest change : pendingChanges) 
                    {
                        switch(change.type)
                        {
                            case ChangeRequest.REGISTER:
                                // nothing to do here
                                break;
                            case ChangeRequest.CHANGE_OPS:
                                // change the interest on the key
                                SelectionKey key = change.socket.keyFor(selector);
                                
                                // Hajo: why can this happen?
                                if(key != null)
                                {
                                    key.interestOps(change.ops);
                                }
                                break;
                            default:
                                System.err.println("Error! Bad change type in queue:" + change.type);
                        }
                    }
                    this.pendingChanges.clear();
                }

                System.out.println("Select()");

                // Wait for an event in one of the registered channels
                this.selector.select();

                System.out.println("Process new event keys");

                Iterator selectedKeys = selector.selectedKeys().iterator();
                while (selectedKeys.hasNext())
                {
                    SelectionKey key = (SelectionKey) selectedKeys.next();

                    if(key.isValid())
                    {
                        // Handle the keyed actions
                        if(key.isAcceptable())
                        {
                            accept(key);
                        }
                        else if(key.isReadable())
                        {
                            read(key);
                        }
                        else if(key.isWritable())
                        {
                            write(key);
                        }
                    }
                    
                    selectedKeys.remove();
                }
            }
            catch (Exception e)
            {
                // report problem, but try to go on.
                e.printStackTrace();
            }
        }
    }

    
    private void accept(SelectionKey key) throws IOException
    {
        ServerSocketChannel serverSocketChannel = (ServerSocketChannel) key.channel();

        // Accept the connection and make it non-blocking
        SocketChannel socketChannel = serverSocketChannel.accept();
        socketChannel.configureBlocking(false);

        // The new channel must be registered with our Selector
        socketChannel.register(this.selector, SelectionKey.OP_READ);
    }

    
    private void read(SelectionKey key) throws IOException
    {
        SocketChannel socketChannel = (SocketChannel) key.channel();
        readBuffer.clear();

        // Attempt to read off the channel
        int bytesRead;
        try
        {
            bytesRead = socketChannel.read(readBuffer);
        }
        catch (IOException e)
        {
            // The remote forcibly closed the connection, cancel
            // the selection key and close the channel.
            key.cancel();
            socketChannel.close();
            
            return;
        }

        if (bytesRead == -1)
        {
            // Remote entity shut the socket down cleanly. Do the
            // same from our end and cancel the channel.
            key.channel().close();
            key.cancel();
            
            return;
        }

        // hand the data to the worker thread for processing
        this.worker.processData(this, socketChannel, readBuffer.array(), bytesRead);
    }

    
    private void write(SelectionKey key) throws IOException
    {
        SocketChannel socketChannel = (SocketChannel) key.channel();

        synchronized (this.pendingData)
        {
            List queue = (List) this.pendingData.get(socketChannel);

            // Write until there's no more data ...
            while (!queue.isEmpty())
            {
                ByteBuffer buf = (ByteBuffer) queue.get(0);
                socketChannel.write(buf);
                if (buf.remaining() > 0)
                {
                    // ... or the socket's buffer fills up
                    break;
                }
                queue.remove(0);
            }

            if (queue.isEmpty())
            {
                // We wrote away all data, so we're no longer interested
                // in writing on this socket. Switch back to reading
                key.interestOps(SelectionKey.OP_READ);
            }
        }
    }

    
    private Selector initSelector() throws IOException
    {
        // Create a new selector
        Selector socketSelector = SelectorProvider.provider().openSelector();

        // Create a new non-blocking server socket channel
        serverChannel = ServerSocketChannel.open();
        serverChannel.configureBlocking(false);

        // Bind the server socket to the specified address and port
        InetSocketAddress isa = new InetSocketAddress(this.host, this.port);
        serverChannel.socket().bind(isa);

        // Register the server socket channel, indicating an interest in 
        // accepting new connections
        serverChannel.register(socketSelector, SelectionKey.OP_ACCEPT);

        return socketSelector;
    }

    
    public static void main(String[] args)
    {
        try
        {
            ServerWorker worker = new MapWorker();
            new Thread(worker).start();
            
            Server server = new Server(null, 9194, worker);
            new Thread(server).start();
        }
        catch (IOException e)
        {
            e.printStackTrace();
        }
    }
}
