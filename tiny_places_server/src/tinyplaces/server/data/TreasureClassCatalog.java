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
public class TreasureClassCatalog 
{
    private static final HashMap<String, TreasureClass> allTresureClasses = new HashMap<String, TreasureClass> (256);
    
    public static TreasureClass get(String id)
    {
        return allTresureClasses.get(id);
    }
    
    public static void init() throws IOException
    {
        InputStream is = Class.class.getClass().getResourceAsStream("/tinyplaces/resources/treasure_classes.csv");
        InputStreamReader sr = new InputStreamReader(is);
        BufferedReader reader = new BufferedReader(sr);

        String line;
        
        // Read the column headers - not used at the moment
        line = reader.readLine();
        
        while((line = reader.readLine()) != null)
        {
            int i = 0;
            String [] parts = line.split(",");
            String id = parts[i++];

            TreasureClass treasureClass = get(id);
            
            if(treasureClass == null)
            {
                treasureClass = new TreasureClass();
                treasureClass.id = id;
            }
                    
            // chances are percent strings in the input file, normalize to [0..1]
            treasureClass.chances.add(Integer.parseInt(parts[i++]) * 0.01);
            treasureClass.items.add(parts[i++]);

            allTresureClasses.put(id, treasureClass);
        }
        
        reader.close();
        Logger.getLogger(TreasureClassCatalog.class.getName()).log(Level.INFO, "Loaded {0} treasure classes.", allTresureClasses.size());                        
    }    
}
