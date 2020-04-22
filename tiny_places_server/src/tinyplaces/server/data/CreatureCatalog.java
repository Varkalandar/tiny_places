package tinyplaces.server.data;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.HashMap;

/**
 *
 * @author hjm
 */
public class CreatureCatalog 
{
    private static final HashMap<String, Creature> allSpells = new HashMap<String, Creature> (256);
    
    public static Creature get(String id)
    {
        return allSpells.get(id);
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
            creature.minLife = Integer.parseInt(parts[i++]);
            creature.maxLife = Integer.parseInt(parts[i++]);

            creature.resistance[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_FIRE] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_COLD] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_LIGHT] = Integer.parseInt(parts[i++]);
            creature.resistance[Damage.TYPE_CHAOS] = Integer.parseInt(parts[i++]);
            
            creature.pattern = parts[i++];
            creature.speed = Integer.parseInt(parts[i++]);
            allSpells.put(parts[0], creature);
        }
    }
    
}
