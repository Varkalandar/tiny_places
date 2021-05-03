package tinyplaces.server.data;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.HashMap;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 *
 * @author hjm
 */
public class CreatureCatalog 
{
    private static final HashMap<String, Creature> allCreatures = new HashMap<String, Creature> (256);
    
    public static Creature get(String id)
    {
        return allCreatures.get(id);
    }
    
    public static void init() throws IOException
    {
        InputStream is = Class.class.getClass().getResourceAsStream("/tinyplaces/resources/creatures.csv");
        InputStreamReader sr = new InputStreamReader(is);
        BufferedReader reader = new BufferedReader(sr);

        String line;
        
        // Read the column headers - not used at the moment
        line = reader.readLine();
        
        while((line = reader.readLine()) != null)
        {
            String [] parts = line.split(",");
            Creature creature = new Creature();
            
            int i = 0;
            creature.id = parts[i++];
            creature.displayName = parts[i++];
            creature.tile = Integer.parseInt(parts[i++]);
            creature.frames = Integer.parseInt(parts[i++]);
            creature.phases = Integer.parseInt(parts[i++]);

            creature.minLife = Integer.parseInt(parts[i++]);
            creature.maxLife = Integer.parseInt(parts[i++]);

            creature.resistance[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_FIRE] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_COLD] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_LIGHT] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_CHAOS] = Integer.parseInt(parts[i++]);
            
            creature.spellId = parts[i++];
            
            String pattern = parts[i++];
            creature.pattern = "-".equals(pattern) ? null : pattern;
            creature.speed = Integer.parseInt(parts[i++]);

            creature.color = parts[i++];
            creature.scale = Float.parseFloat(parts[i++]);
            creature.treasureClasses = parts[i++];
            
            allCreatures.put(parts[0], creature);
        }
        
        reader.close();
        Logger.getLogger(CreatureCatalog.class.getName()).log(Level.INFO, "Loaded {0} creatures.", allCreatures.size());                        
    }    
}
