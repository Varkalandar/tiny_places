package tinyplaces.server;

import java.nio.channels.SocketChannel;

/**
 * Data container for queuing interest changed on channel keys
 */ 
public class ChangeRequest
{
    // Change types
    public static final int REGISTER = 1;
    public static final int CHANGE_OPS = 2;
    
    // Change data
    public final SocketChannel socket;
    public final int type;
    public final int ops;

    public ChangeRequest(SocketChannel socket, int type, int ops)
    {
        this.socket = socket;
        this.type = type;
        this.ops = ops;
    }
}
