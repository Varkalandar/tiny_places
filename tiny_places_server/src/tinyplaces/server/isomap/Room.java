package tinyplaces.server.isomap;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Set;
import java.util.logging.Level;
import java.util.logging.Logger;
import tinyplaces.server.CommandWorker;
import tinyplaces.server.Server;
import tinyplaces.server.ServerDataEvent;
import tinyplaces.server.data.Creature;
import tinyplaces.server.data.CreatureCatalog;
import tinyplaces.server.data.Damage;
import tinyplaces.server.data.Item;
import tinyplaces.server.data.ItemBuilder;
import tinyplaces.server.data.Population;
import tinyplaces.server.data.PopulationsCatalog;
import tinyplaces.server.data.Spell;
import tinyplaces.server.data.SpellCatalog;
import tinyplaces.server.data.TreasureClass;
import tinyplaces.server.data.TreasureClassCatalog;
import tinyplaces.server.isomap.actions.Action;


/**
 * A room (map segment)
 * @author Hj. Malthaner
 */
public class Room 
{
    private static final HashMap<String, Room> rooms = new HashMap<String, Room>(64);

    private int nextObjectId = 1;
    
    private final HashMap <Integer, Mob> patches = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> mobs = new HashMap<Integer, Mob>();
    private final HashMap <Integer, Mob> clouds = new HashMap<Integer, Mob>();
    
    private final ArrayList<Action> actions = new ArrayList<Action>(256);
    private final ArrayList<Action> actionsToAdd = new ArrayList<Action>(256);
    private final ArrayList<CreatureGroup> groups = new ArrayList<CreatureGroup>(32);

    private CommandWorker commandWorker;
    private Server server;
    
    public final String name;
    public final String backdrop;
    
    
    public static HashMap<String, Room> getRooms()
    {
        return rooms;
    }


    public Room(String name, String backdrop)
    {
        this.name = name;
        this.backdrop = backdrop;
        rooms.put(name, this);
    }

    
    public ArrayList<Action> getActions()
    {
        actions.addAll(actionsToAdd);
        actionsToAdd.clear();
        
        return actions;
    }
    
    
    public void addAction(Action move) 
    {
        // due to threading and the fact that some actions want to add
        // new actions while being proccess (ConcurrentModification)
        // new actions go into a queue first and will be added at a
        // proper time

        actionsToAdd.add(move);
    }

    
    public HashMap <Integer, Mob> getLayerMap(int layer)
    {
        switch(layer)
        {
            case 1:
                return patches;
            case 3:
                return mobs;
            case 5:
                return clouds;
            default:
                Logger.getLogger(Room.class.getName()).log(Level.SEVERE, "No such layer: {0}", layer);
                return null;
        }
    }
    
    
    public int getNextObjectId()
    {
        return nextObjectId ++;
    }

    
    public void addMob(int layer, Mob mob)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        
        synchronized(lmap)
        {
            lmap.put(mob.id, mob);
        }
    }
    
    
    public Mob getMob(int layer, int id)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        return lmap.get(id);
    }

    
    public Mob removeMob(int layer, int id)
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);
        Mob mob;
    
        for(CreatureGroup group : groups)
        {
            group.remove(id);
        }
        
        synchronized(lmap)
        {
            mob = lmap.remove(id);
        }
        return mob;
    }


    public void save(String filename) 
    {
        try 
        {
            File file = new File("maps", filename);
            FileWriter writer = new FileWriter(file);
            
            writer.write("v10\n");
            writer.write(name + "\n");
            writer.write(backdrop + "\n");
            saveLayer(writer, 1);
            saveLayer(writer, 3);
            saveLayer(writer, 5);
            
            writer.close();
        }
        catch (IOException ex) 
        {
            Logger.getLogger(Room.class.getName()).log(Level.SEVERE, null, ex);
        }
    }
    
    
    private void saveLayer(Writer writer, int layer) throws IOException 
    {
        HashMap <Integer, Mob> lmap = getLayerMap(layer);

        Set <Integer> keys = lmap.keySet();

        for(Integer i : keys)
        {
            Mob mob = lmap.get(i);
            if(mob.type == Mob.TYPE_PROP)
            {
                // id will not be saved but set freshly on loading the map
                String line = "" + layer + "," +
                    mob.tile + "," +
                    mob.x + "," +
                    mob.y + "," +
                    mob.scale + "," +
                    mob.color + "\n";

                writer.write(line);
            }
        }
    }


    public Mob makeMob(String [] parts)
    {
        assert(parts.length == 8);
        
        int layer = Integer.parseInt(parts[1]);
        int tile = Integer.parseInt(parts[2]);
        int frames = Integer.parseInt(parts[3]);
        int phases = Integer.parseInt(parts[4]);
        int x = Integer.parseInt(parts[5]);
        int y = Integer.parseInt(parts[6]);
        float scale = Float.parseFloat(parts[7]);
        String color = parts[8].trim();

        return makeMob(layer, tile, frames, phases, x, y, scale, color, Mob.TYPE_PROP);
    }
    
    
    public Mob makeMob(int layer, int tile, int frames, int phases, int x, int y, float scale, String color, int type)
    {
        int id = getNextObjectId();
        
        Mob mob = new Mob();
        mob.id = id;
        mob.tile = tile;
        mob.frames = frames;
        mob.phases = phases;
        mob.x = x;
        mob.y = y;
        mob.scale = scale;
        mob.color = color;
        mob.type = type;
        
        addMob(layer, mob);
        
        return mob;
    }

    
    public List <Mob> makeMobGroup(String id, int minCount, int maxCount, int centerX, int centerY, int spacing)
    {
        int count = minCount + (int)(Math.random() * (maxCount - minCount + 1));
        
        ArrayList <Mob> result = new ArrayList<Mob>(count);        
        for(int i=0; i<count; i++)
        {
            int x, y, tries = 0;
            boolean ok;
            
            // don't place mobs in the same spot if possible
            // 10 tries will be made to find a clear spot
            do
            {
                x = centerX + (int)(spacing * (Math.random() * 10 - 5));
                y = centerY + (int)((spacing / 2) * (Math.random() * 10 - 5));

                ok = true;
                for(Mob mob : result)
                {
                    int dx = mob.x - x;
                    int dy = mob.y - y;
                    int d = dx * dx + dy * dy;
                    
                    // must be at least 15 units from each other
                    ok = d < 225;
                }
                tries ++;
            } while(!ok && tries < 10);
                    
            Creature creature = CreatureCatalog.get(id);
            Mob mob = makeMob(3, creature.tile, creature.frames, creature.phases, 
                              x, y, creature.scale, creature.color, Mob.TYPE_CREATURE);
            mob.creature = creature.create();
            mob.nextAiTime = System.currentTimeMillis() + (int)(Math.random() * 10000);
            
            result.add(mob);
        }
        
        CreatureGroup creatureGroup = new CreatureGroup(result, centerX, centerY);
        groups.add(creatureGroup);

        return result;
    }
    
    /*
     * Todo: Move this to a better place someday?
     */
    synchronized public void aiCall()
    {
        long time = System.currentTimeMillis();
        
        for(CreatureGroup group : groups)
        {
            for(Mob mob : group.creatures)
            {
                if(mob.nextAiTime < time)
                {
                    // fire at a player?
                    if(Math.random() < 0.25)
                    {
                        ArrayList<Mob> moblist = new ArrayList<Mob>  (mobs.values());
                        // find a player
                        for(Mob target : moblist)
                        {
                            if(target.type != Mob.TYPE_PROJECTILE && target.type == Mob.TYPE_PLAYER)
                            {
                                Spell spell = SpellCatalog.get(mob.creature.spellId);
                                if(spell != null)
                                {
                                    commandWorker.fireProjectile(this, mob, 3, target.x, target.y, spell);
                                }
                            }
                        }
                        mob.nextAiTime = time + 1000 + (int)(Math.random() * 1000);
                    }
                    else if(mob.creature.pattern != null)
                    {
                        // move
                        int x, y, len;
                        int count = 0;

                        do
                        {
                            x = mob.x + 100 - (int)(Math.random() * 200);
                            y = mob.y + 100 - (int)(Math.random() * 200);

                            int dx = (x - group.cx);
                            int dy = (y - group.cy);

                            len = dx * dx + (dy * dy) * 4;
                            count ++;

                            // System.err.println("len=" + len);
                        } while(len > 100 * 100 && count < 5);

                        if(count >= 5)
                        {
                            x = group.cx + 50 - (int)(Math.random() * 100);
                            y = group.cy + 50 - (int)(Math.random() * 100);
                        }

                        // System.err.println("id=" + creature.id + "moves to " + x + ", " + y);
                        commandWorker.doMove(null, this, mob.id, 3, x, y, 
                                             mob.creature.speed, mob.creature.pattern);

                        mob.nextAiTime = time + 3000 + (int)(Math.random() * 2000);
                    }
                }
            }
        }
    }
    
    public void setCommandWorker(CommandWorker commandWorker) 
    {
        this.commandWorker = commandWorker;
    }

    public void setServer(Server server) 
    {
        this.server = server;
    }

    public Server getServer() 
    {
        return server;
    }

    /**
     * Transit player to a new room
     */ 
    void transit(ServerDataEvent dataEvent, Mob mob, String roomname, int newx, int newy) 
    {
        commandWorker.transit(dataEvent, mob, this, roomname, newx, newy);
    }

    public void populateRoom(ServerDataEvent dataEvent, String roomname)
    {
        List<Population> populations = PopulationsCatalog.get(roomname);
        if(populations != null)
        {
            for(Population population : populations)
            {
                List <Mob> mobs = 
                        makeMobGroup(population.creatureId,
                                     population.minCount, population.maxCount,
                                     population.x, population.y, population.spacing);
                commandWorker.addMobGroup(dataEvent, this, mobs, 3);    
            }
        }
    }
    
    /**
     * Result is indexed by distance square 
     */
    HashMap <Integer, Mob> findMobsNear(int x, int y, int limit) 
    {
        HashMap <Integer, Mob> result = new HashMap<Integer, Mob>(64);
        int dmax = limit * limit;
        
        for(Mob mob : mobs.values())
        {
            int dx = mob.x - x;
            int dy = mob.y - y;
            
            int d = dx * dx + dy * dy; 
            
            if(d <= dmax)
            {
                result.put(d, mob);
            }
        }
        
        return result;
    }

    synchronized void handleHit(Mob projectile, Mob target) 
    {
        Spell spell = projectile.spell;
        Creature creature = target.creature;

        // todo - environment hits?
        if(spell != null && creature != null)
        {
            int damage = Damage.calculate(creature, spell);

            System.err.println("Room: " + creature.displayName + " was hit by " + spell.displayName + " for " + damage + " damage.");


            creature.actualLife -= damage;

            if(creature.actualLife < 0)
            {
                commandWorker.kill(target, this);
                handleDrops(target);
            }
        }
    }

    private void handleDrops(Mob target) 
    {
        String [] parts = target.creature.treasureClasses.split(" ");
        for(String tc : parts)
        {
            TreasureClass treasure = TreasureClassCatalog.get(tc);
            for(int i=0; i<treasure.chances.size(); i++)
            {
                if(Math.random() < treasure.chances.get(i))
                {
                    String itemId = treasure.items.get(i);
                    Item item = ItemBuilder.create(itemId);
                    item.mobId = getNextObjectId();
                    item.position.x = target.x + (int)(Math.random() * 40 - 20);
                    item.position.y = target.y + (int)(Math.random() * 20 - 10);
                    item.where = Item.ON_MAP;

                    commandWorker.dropItem(this, item);
                }
            }
        }
    }

}
