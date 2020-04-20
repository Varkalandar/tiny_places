package tinyplaces.server.data;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.HashMap;

/**
 * Read and manage spell data.
 * 
 * @author hjm
 */
public class Spells 
{
    private static final HashMap<String, Spell> allSpells = new HashMap<String, Spell> (256);
    
    public static Spell get(String id)
    {
        return allSpells.get(id);
    }
    
    public static void init() throws IOException
    {
        InputStream is = Class.class.getClass().getResourceAsStream("/tinyplaces/resources/spells.csv");
        InputStreamReader sr = new InputStreamReader(is);
        BufferedReader reader = new BufferedReader(sr);
        
        
        String line;
        
        // Read the column headers - not used at the moment
        line = reader.readLine();
        
        while((line = reader.readLine()) != null)
        {
            String [] parts = line.split(",");
            Spell spell = new Spell();
            
            spell.displayName = parts[1];
            spell.min[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[2]);
            spell.max[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[3]);
            spell.min[Damage.TYPE_FIRE] = Integer.parseInt(parts[4]);
            spell.max[Damage.TYPE_FIRE] = Integer.parseInt(parts[5]);
            spell.min[Damage.TYPE_COLD] = Integer.parseInt(parts[6]);
            spell.max[Damage.TYPE_COLD] = Integer.parseInt(parts[7]);
            spell.min[Damage.TYPE_LIGHT] = Integer.parseInt(parts[8]);
            spell.max[Damage.TYPE_LIGHT] = Integer.parseInt(parts[9]);
            spell.min[Damage.TYPE_CHAOS] = Integer.parseInt(parts[10]);
            spell.max[Damage.TYPE_CHAOS] = Integer.parseInt(parts[11]);
            
            allSpells.put(parts[0], spell);
        }
    }
}
