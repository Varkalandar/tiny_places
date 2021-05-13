package tinyplaces.server.isomap;

import java.awt.Point;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Set;
import java.util.SortedMap;
import tinyplaces.server.data.BaseItem;
import tinyplaces.server.data.Item;
import tinyplaces.server.data.Transition;
import tinyplaces.server.data.TransitionCatalog;
import tinyplaces.server.isomap.actions.Action;
import tinyplaces.server.isomap.actions.Move;

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
            
            if(mob.type == Mob.TYPE_PROJECTILE)
            {
                checkProjectileHit(room, mob);
            }
            else
            {
                // Players can pick up items
                checkPlayerPickup(room, mob, move);

                // Player moves can result in a map change
                checkTransitions(room, mob, move);
            }
        }
    }

    private void checkProjectileHit(Room room, Mob mob)
    {
        int radius = 20;
        SortedMap <Integer, Mob> map = room.findMobsNear(mob.x, mob.y, radius);
        
        if(map.size() > 0)
        {
            Integer nearest = map.firstKey();        
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
        }
        else
        {
            System.err.println("MapWorker: projectile hit nothing.");
        }
    }

    
    private void checkPlayerPickup(Room room, Mob mob, Move move) 
    {        
        SortedMap <Integer, Item> map = room.findItemsNear(mob.x, mob.y, 20);

        if(map.size() > 0)
        {       
            Integer nearest = map.firstKey();
            Item item = map.get(nearest);
        
            if(BaseItem.CLASS_POWERUP.equals(item.baseItem.iclass))
            {
                room.applyPowerup(move.client, item);
            }
            else
            {
                room.handlePickup(move.client, item);
            }
        }
    }
    
    
    private Transition checkTransitions(Room room, Mob mob, Move move) 
    {        
        List<Transition> transitions = TransitionCatalog.get(room.name);
        
        // System.err.println("room=" + room.name + " has " + transitions);        
        if(transitions != null)
        {
            for(Transition t : transitions)
            {
                int dx = move.x - t.fromX;
                int dy = move.y - t.fromY;
                int d2 = dx * dx + dy * dy;

                // monsters have no data event ... cannot transit to another room
                if(d2 < 250 && move.client != null)
                {
                    room.transit(move.client, mob, t.toMap, t.toX, t.toY);
                    return t;
                }
            }
        }
        
        return null;
    }
}
