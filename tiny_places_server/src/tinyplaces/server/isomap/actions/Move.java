package tinyplaces.server.isomap.actions;

import tinyplaces.server.ServerDataEvent;
import tinyplaces.server.isomap.Client;
import tinyplaces.server.isomap.Mob;
import tinyplaces.server.isomap.Room;

/**
 * Move a mob to a given position over time.
 * 
 * @author hjm
 */
public class Move implements Action
{
    public final Client client;
    public final Mob mob;
    public final int layer;
    public final int x;
    public final int y;
    public final int speed;
    private boolean done;
    private double xp;
    private double yp;
    

    @Override
    public Mob getMob()
    {
        return mob;
    }


    public Move(Client client, Mob mob, int layer, int x, int y, int speed)
    {
        this.client = client;
        this.mob = mob;
        this.layer = layer;
        this.x = x;
        this.y = y;
        this.speed = speed;
        this.xp = mob.x;
        this.yp = mob.y;
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
        double dx = x - xp;
        double dy = y - yp;

        double len = Math.sqrt(dx * dx + dy * dy);
  
        // print("dx=" .. dx .. " dy=" .. dy .. " len="..len)
  
        double steplen = dt * speed * 0.001; // dt is milliseconds
  
        if(len > steplen)
        {
            double nx = dx/len * steplen;
            double ny = dy/len * steplen;
            xp += nx;
            yp += ny;
            
            mob.x = (int)(xp + 0.5);
            mob.y = (int)(yp + 0.5);

            /*
            if(mob.id == 47)
            {
                System.err.println("mob.x=" + mob.x + " mob.y=" + mob.y);
            }
            */
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
