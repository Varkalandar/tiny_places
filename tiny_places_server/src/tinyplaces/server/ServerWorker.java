package tinyplaces.server;

import java.nio.channels.SocketChannel;

/**
 *
 * @author Hj. Malthaner
 */
public interface ServerWorker extends Runnable
{
    public void processData(Server server, SocketChannel socket, byte[] data, int count);    
}
