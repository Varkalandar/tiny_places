package tinyplaces.server;

import java.nio.channels.SocketChannel;

public class ServerDataEvent
{
    public final Server server;
    public final SocketChannel socket;
    public final byte[] data;

    
    public ServerDataEvent(Server server, SocketChannel socket, byte[] data)
    {
        this.server = server;
        this.socket = socket;
        this.data = data;
    }

    
    public ServerDataEvent(Server server, SocketChannel socket, String responseData)
    {
        this(server, socket, responseData.getBytes());
    }
}