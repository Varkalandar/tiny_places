package tinyplaces.server.isomap;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Set;
import tinyplaces.server.isomap.actions.Move;
import tinyplaces.server.isomap.actions.Action;

/**
 * Worker thread to process ongoing actions.
 * 
 * @author hjm
 */
public class MapWorker implements Runnable 
{
    @Override
    public void run() 
    {
        long lastTime = System.currentTimeMillis();
        
        while(true)
        {
            try
            {
                Thread.sleep(100);
            }
            catch(InterruptedException iex)
            {
                System.err.println("MapWorker: Interrupt during wait:" + iex);
            }
            
            long now = System.currentTimeMillis();
            int dt = (int)(now - lastTime);
            
            HashMap<String, Room> roomsMap = Room.getRooms();

            // System.err.println("MapWorker: dt=" + dt);
            // System.err.println("MapWorker: room count:" + rooms.size());

            ArrayList <Room> rooms = new ArrayList<Room>(roomsMap.values());
                    
            for(Room room : rooms)
            {
                List <Action> actions = room.getActions();
                List <Action> killList = new ArrayList<Action>();

                // System.err.println("MapWorker: action count:" + actions.size());
                
                synchronized(actions)
                {
                    for(Action action : actions)
                    {
                       action.process(room, dt);
                       if(action.isDone())
                       {
                           killList.add(action);
                       }
                    }
                }

                for(Action action : killList)
                {
                    processActionResult(room, action);
                }
                
                synchronized(actions)
                {
                    actions.removeAll(killList);
                }
                
                room.aiCall();
            }
            
            lastTime = now;
        }
    }

    private void processActionResult(Room room, Action action) 
    {
        Mob mob = action.getMob();
        
        if(action instanceof Move)
        {
            Move move = (Move)action;
            
            // Player moves can result in a map change
            if("Lobby".equals(room.name))
            {
                {
                    // change to wasteland
                    int dx = move.x - 837;
                    int dy = move.y - 168;
                    int d2 = dx * dx + dy * dy;

                    // monsters have no data event ... cannot transit to another room
                    if(d2 < 250 && move.dataEvent != null)
                    {
                        room.transit(move.dataEvent, mob, "wasteland_and_pond", 360, 480);
                    }
                }
                {
                    // change to desert
                    int dx = move.x - 392;
                    int dy = move.y - 144;
                    int d2 = dx * dx + dy * dy;

                    // monsters have no data event ... cannot transit to another room
                    if(d2 < 250 && move.dataEvent != null)
                    {
                        room.transit(move.dataEvent, mob, "desert", 867, 466);
                    }
                }
            }        
        
            if(mob.type == Mob.TYPE_PROJECTILE)
            {
                int radius = 20;
                HashMap <Integer, Mob> map = room.findMobsNear(mob.x, mob.y, radius);

                Set <Integer> distances = map.keySet();

                int nearest = radius * radius;

                for(Integer i : distances)
                {
                    if(i < nearest)
                    {
                        nearest = i;
                    }
                }

                Mob target = map.get(nearest);

                if(target != null)
                {
                    System.err.println("MapWorker: projectile hit mob id=" + target.id);

                    // for now, don't kill the player ...
                    if(target.type != Mob.TYPE_PLAYER)
                    {
                        room.handleHit(mob, target);
                    }
                }
                else
                {
                    System.err.println("MapWorker: projectile hit nothing.");
                }
            }
        }
    }
}
