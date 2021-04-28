package tinyplaces.server.isomap;

import java.awt.Point;
import java.awt.Rectangle;
import java.nio.channels.SocketChannel;
import java.util.ArrayList;
import tinyplaces.server.data.Item;

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
    
    private ArrayList<Item> equipment = new ArrayList<Item>(64);

    public final SocketChannel socket;
    
    public Client(SocketChannel socket) 
    {
        this.socket = socket;
    }

    
    public Room getCurrentRoom()
    {
        return currentRoom;
    }
    
    public void setCurrentRoom(Room room)
    {
        this.currentRoom = room;
    }
    
    private Item intersectsAnyEquipped(Item item)
    {
        Rectangle itemRect = 
                new Rectangle(item.position.x, item.position.y, 
                              item.baseItem.width, item.baseItem.height);

        for(Item equipped : equipment)
        {
            Rectangle equippedRect = 
                new Rectangle(equipped.position.x, equipped.position.y, 
                              equipped.baseItem.width, equipped.baseItem.height);

            if(equippedRect.intersects(itemRect))
            {
                return equipped;
            }                
        }
        
        return null;
    }
    
    /**
     * Adds an item to the inventory or one of the equipment slots.
     * If there is an item that obstructs the operation, the obstructing item
     * will be returned.
     * @param item The item to add
     * @return null or the item that blocked the operation.
     */
    public Item addItem(Item item)
    {
        if(item.where >= 0)
        {
            // item for a slot
            for(Item equipped : equipment)
            {
                if(equipped.where == item.where)
                {
                    return equipped;
                }
                else
                {
                    equipment.add(item);
                    return null; // this means ok
                }
            }
        }
        else if(item.where == Item.IN_INVENTORY)
        {
            if(intersectsAnyEquipped(item) == null)
            {
                equipment.add(item);
            }
        }
        else
        {
            throw new IllegalArgumentException("Illegal value for 'item.where':" + item.where);
        }
        return null;
    }
    
    /**
     * Find an empty spot for the item in the inventory. This method modifies
     * item.position.x and item.position.y!
     * 
     * @param item The item
     * @return Point of the location or null if no suitable location was found
     */
    public Point findSuitableLocation(Item item)
    {
        for(int y = 0; y < 6; y++)
        {
            for(int x = 0; x < 32; x++)
            {
                item.position.x = x;
                item.position.y = y;
                if(intersectsAnyEquipped(item) == null)
                {
                    return new Point(x, y);
                }
            }
        }
        return null;
    }
}
