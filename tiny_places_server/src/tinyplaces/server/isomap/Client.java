package tinyplaces.server.isomap;

/**
 *
 * @author hjm
 */
public class Client 
{
    private Room currentRoom;
    
    public Room getCurrentRoom()
    {
        return currentRoom;
    }
    
    public Client(Room room)
    {
        currentRoom = room;
    }
}
