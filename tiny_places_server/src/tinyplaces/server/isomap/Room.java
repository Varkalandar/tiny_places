package tinyplaces.server.isomap;

import java.util.HashMap;

/**
 * A room (map segment)
 * @author Hj. Malthaner
 */
public class Room 
{
    public static final Room LOBBY = new Room();
    private int nextObjectId = 1;
    
    private HashMap <Integer, Mob> mobs = new HashMap<Integer, Mob>();
    
    public int getNextObjectId()
    {
        return nextObjectId ++;
    }

    public void addMob(Mob mob)
    {
        mobs.put(mob.id, mob);
    }
    
    public Mob getMob(int id)
    {
        return mobs.get(id);
    }

    public void removeMob(int id)
    {
        mobs.remove(id);
    }
    
}
