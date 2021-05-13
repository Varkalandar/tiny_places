package tinyplaces.server.isomap;

import java.awt.Point;
import java.awt.Rectangle;
import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.nio.channels.SocketChannel;
import java.util.ArrayList;
import java.util.logging.Level;
import java.util.logging.Logger;
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
    public final Stat [] stats = new Stat[6];
    
    public final SocketChannel socket;
    
    public Client(String id, SocketChannel socket)
    {
        Logger.getLogger(Client.class.getName()).log(Level.INFO, "Loading player <{0}>", id);
        this.socket = socket;
        try {
            loadPlayerData(id);
        } catch (IOException ex) {
            Logger.getLogger(Client.class.getName()).log(Level.SEVERE, null, ex);
        }
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

    private void loadPlayerData(String id) throws IOException
    {
        FileReader fr = new FileReader("players/" + id.toLowerCase() + ".ini");
        BufferedReader reader = new BufferedReader(fr);
        
        String line;
        while((line = reader.readLine()) != null)
        {
            String [] parts = line.split(",");
            int statIndex = Integer.parseInt(parts[0]);
            int statMin = Integer.parseInt(parts[1]);
            int statMax = Integer.parseInt(parts[2]);
            int statValue = Integer.parseInt(parts[3]);
            
            if(statIndex < stats.length)
            {
                stats[statIndex] = new Stat();
                stats[statIndex].min = statMin;
                stats[statIndex].max = statMax;
                stats[statIndex].value = statValue;
            }
        }
        
        reader.close();
    }
    
    /**
     * Life, Energy etc.
     */
    public static class Stat
    {
        public int min;
        public int max;
        public int value;
    }
}
