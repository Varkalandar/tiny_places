package tinyplaces.server.data;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.URL;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 *
 * @author hjm
 */
public class PopulationsCatalog 
{
    private static final HashMap<String, ArrayList<Population>> allPopulations = new HashMap<String, ArrayList<Population>> (1024);
    
    public static List<Population> get(String map)
    {
        return allPopulations.get(map);
    }
    
    public static void init(URL resource) throws IOException
    {
        InputStream is = resource.openStream();
        InputStreamReader sr = new InputStreamReader(is);
        BufferedReader reader = new BufferedReader(sr);
        
        String line;
        
        // Read the column headers - not used at the moment
        line = reader.readLine();
        
        while((line = reader.readLine()) != null)
        {
            String [] parts = line.split(",");
            Population population = new Population();
            int i = 0;
            
            String mapId = parts[i++];
            population.creatureId = parts[i++];
            population.minCount = Integer.parseInt(parts[i++]);
            population.maxCount = Integer.parseInt(parts[i++]);
            population.x = Integer.parseInt(parts[i++]);
            population.y = Integer.parseInt(parts[i++]);
            population.spacing = Integer.parseInt(parts[i++]);
            
            ArrayList<Population> populations =  allPopulations.get(mapId);
            if(populations == null)
            {
                populations = new ArrayList<Population>(16);
                allPopulations.put(mapId, populations);
            }
            
            populations.add(population);
        }

        reader.close();
        Logger.getLogger(PopulationsCatalog.class.getName()).log(Level.INFO, "Loaded {0} map populations.", allPopulations.size());                        
    }
}
