package tinyplaces.server.isomap;

import java.util.ArrayList;

/**
 *
 * @author hjm
 */
public class CreatureGroup 
{
    ArrayList <Mob> creatures = new ArrayList<Mob> (32);
    
    // Group center x and y - the group should move as a whole
    int cx;
    int cy;
    
    CreatureGroup(ArrayList<Mob> mobs, int cx, int cy)
    {
        creatures.addAll(mobs);
        this.cx = cx;
        this.cy = cy;
    }

    void remove(int id) 
    {
        for(int i=0; i<creatures.size(); i++)
        {
            if(creatures.get(i).id == id)
            {
                creatures.remove(i);
                return;
            }
        }
    }
}
