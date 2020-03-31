package tinyplaces.server.isomap;

/**
 * Client representation on server side.
 * 
 * @author hjm
 */
public class Client 
{
    private Room currentRoom;

    // the player avatar
    public Mob mob;
    
    public Room getCurrentRoom()
    {
        return currentRoom;
    }
    
    public void setCurrentRoom(Room room)
    {
        this.currentRoom = room;
    }
    
    public Client(Room room)
    {
        currentRoom = room;
    }
}
