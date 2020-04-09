package tinyplaces.server.isomap.actions;

import tinyplaces.server.isomap.Mob;
import tinyplaces.server.isomap.Room;

/**
 *
 * @author hjm
 */
public class Move implements MapAction
{
    private final Mob mob;
    private final int layer;
    private final int x;
    private final int y;
    private final int speed;
    private boolean done;
    

    public Mob getMob()
    {
        return mob;
    }


    public Move(Mob mob, int layer, int x, int y, int speed)
    {
        this.mob = mob;
        this.layer = layer;
        this.x = x;
        this.y = y;
        this.speed = speed;
        done = false;
    }

    
    @Override
    public boolean isDone()
    {
        return done;
    }
    
    
    @Override
    public void process(Room room, int dt)
    {
         // todo: make projectiles travel and expire properly
        // room.deleteMob(layer, projectile.id);
        
        int dx = x - mob.x;
        int dy = y - mob.y;

        double len = Math.sqrt(dx * dx + dy * dy);
  
        // print("dx=" .. dx .. " dy=" .. dy .. " len="..len)
  
        double steplen = dt * speed * 0.001; // dt is milliseconds
  
        if(len > steplen)
        {
            double nx = dx/len * steplen;
            double ny = dy/len * steplen;
  
            mob.x += (int)(nx + 0.5);
            mob.y += (int)(ny + 0.5);

    
            // print("nx=" .. nx .. " ny=" .. ny .. " mob.x="..mob.x .. " mob.y="..mob.y)
        }
        else
        {    
            // eliminate rounding errors
            mob.x = x;
            mob.y = y;
            done = true;
    
            System.err.println("Move done! id=" + mob.id);
    
            if(mob.type == Mob.TYPE_PROJECTILE)
            {
                // System.err.println("Removing expired projectile with id=" + mob.id);
                room.removeMob(layer, mob.id);
            }
        }
    }
}
