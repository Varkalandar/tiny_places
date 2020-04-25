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
public class SpellCatalog 
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
            int i = 0;
            
            spell.id = parts[i++];
            spell.displayName = parts[i++];
            spell.ptype = Integer.parseInt(parts[i++]);
            spell.min[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[i++]);
            spell.max[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[i++]);
            spell.min[Damage.TYPE_FIRE] = Integer.parseInt(parts[i++]);
            spell.max[Damage.TYPE_FIRE] = Integer.parseInt(parts[i++]);
            spell.min[Damage.TYPE_COLD] = Integer.parseInt(parts[i++]);
            spell.max[Damage.TYPE_COLD] = Integer.parseInt(parts[i++]);
            spell.min[Damage.TYPE_LIGHT] = Integer.parseInt(parts[i++]);
            spell.max[Damage.TYPE_LIGHT] = Integer.parseInt(parts[i++]);
            spell.min[Damage.TYPE_CHAOS] = Integer.parseInt(parts[i++]);
            spell.max[Damage.TYPE_CHAOS] = Integer.parseInt(parts[i++]);
            spell.speed = Integer.parseInt(parts[i++]);
            
            allSpells.put(parts[0], spell);
        }

        reader.close();
    }
}
