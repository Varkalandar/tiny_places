package tinyplaces.server.data;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 *
 * @author hjm
 */
public class TransitionCatalog 
{
    private static final HashMap<String, ArrayList<Transition>> allTransitions = new HashMap<String, ArrayList<Transition>> (1024);
    
    public static List<Transition> get(String fromMap)
    {
        return allTransitions.get(fromMap);
    }
    
    public static void init() throws IOException
    {
        InputStream is = Class.class.getClass().getResourceAsStream("/tinyplaces/resources/transitions.csv");
        InputStreamReader sr = new InputStreamReader(is);
        BufferedReader reader = new BufferedReader(sr);
        
        
        String line;
        
        // Read the column headers - not used at the moment
        line = reader.readLine();
        
        while((line = reader.readLine()) != null)
        {
            String [] parts = line.split(",");
            Transition transition = new Transition();
            int i = 0;
            
            transition.id = parts[i++];
            transition.displayName = parts[i++];
            transition.fromMap = parts[i++];
            transition.fromX = Integer.parseInt(parts[i++]);
            transition.fromY = Integer.parseInt(parts[i++]);
            transition.toMap = parts[i++];
            transition.toX = Integer.parseInt(parts[i++]);
            transition.toY = Integer.parseInt(parts[i++]);
            
            System.err.println("fromMap=" + transition.fromMap);
            
            
            ArrayList<Transition> transitions =  allTransitions.get(transition.fromMap);
            if(transitions == null)
            {
                transitions = new ArrayList<Transition>(16);
                allTransitions.put(transition.fromMap, transitions);
            }
            
            transitions.add(transition);
        }

        reader.close();
        Logger.getLogger(TransitionCatalog.class.getName()).log(Level.INFO, "Loaded {0} map transitions.", allTransitions.size());                        
    }
}
