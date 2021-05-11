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
public class ItemCatalog 
{
    private static final HashMap<String, BaseItem> allBaseItems = new HashMap<String, BaseItem> (256);
    
    
    public static BaseItem get(String id)
    {
        BaseItem baseItem = allBaseItems.get(id);
        if(baseItem == null)
        {
            Logger.getLogger(ItemCatalog.class.getName()).log(Level.WARNING, "BaseItem {0} not found in catalog.", id);                
        }
        
        return baseItem;
    }
    
    
    public static void init() throws IOException
    {
        InputStream is = Class.class.getClass().getResourceAsStream("/tinyplaces/resources/items.csv");
        InputStreamReader sr = new InputStreamReader(is);
        BufferedReader reader = new BufferedReader(sr);

        String line;
        
        // Read the column headers - not used at the moment
        line = reader.readLine();
        
        while((line = reader.readLine()) != null)
        {
            String [] parts = line.split(",");
            BaseItem baseItem = new BaseItem();
            
            int i = 0;
            baseItem.id = parts[i++];
            baseItem.displayName = parts[i++];
            baseItem.iclass = parts[i++];
            baseItem.itype = parts[i++];
            baseItem.tile = Integer.parseInt(parts[i++]);
            baseItem.shadow = Integer.parseInt(parts[i++]);
            baseItem.shadowScale = Float.parseFloat(parts[i++]);
            baseItem.width = Integer.parseInt(parts[i++]);
            baseItem.height = Integer.parseInt(parts[i++]);
            baseItem.color = parts[i++];
            baseItem.scale = Float.parseFloat(parts[i++]);
            baseItem.canDrop = "1".equals(parts[i++]);
            baseItem.stackSize = Integer.parseInt(parts[i++]);
            baseItem.baseValue = Integer.parseInt(parts[i++]);
            baseItem.energyDamageMin = Float.parseFloat(parts[i++]);
            baseItem.energyDamageMax = Float.parseFloat(parts[i++]);
            baseItem.physicalDamageMin = Float.parseFloat(parts[i++]);
            baseItem.physicalDamageMax = Float.parseFloat(parts[i++]);
            baseItem.description = parts[i++];

            /*
            baseItem.resistance[Damage.TYPE_PHYSICAL] = Integer.parseInt(parts[i++]);
            baseItem.resistance[Damage.TYPE_FIRE] = Integer.parseInt(parts[i++]);
            baseItem.resistance[Damage.TYPE_COLD] = Integer.parseInt(parts[i++]);
            baseItem.resistance[Damage.TYPE_LIGHT] = Integer.parseInt(parts[i++]);
            baseItem.resistance[Damage.TYPE_CHAOS] = Integer.parseInt(parts[i++]);
*/
            allBaseItems.put(parts[0], baseItem);
        }
        
        reader.close();
        
        Logger.getLogger(ItemCatalog.class.getName()).log(Level.INFO, "Loaded {0} items.", allBaseItems.size());                
    }    
}
