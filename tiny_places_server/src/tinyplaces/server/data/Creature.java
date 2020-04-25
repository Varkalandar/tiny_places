package tinyplaces.server.data;

import java.util.logging.Level;
import java.util.logging.Logger;

/**
 *
 * @author hjm
 */
public class Creature implements Cloneable 
{
    public String id;
    public String displayName;
    public int tile;
    
    public int actualLife;

    public int minLife;
    public int maxLife;
    public String pattern;  // movement pattern
    public int speed;
    int[] resistance = new int [Damage.TYPE_COUNT];
    
    public String spellId;
    
    public String color;
    public float scale;
    
    /**
     * Creature an individual with randomized stats
     * @return 
     */
    public Creature create() 
    {
        Creature c = null;
        try 
        {
            c = (Creature)this.clone();
            
            // The only random stat right now is life
            c.actualLife = minLife + (int)((maxLife - minLife + 1) * Math.random());

        } catch (CloneNotSupportedException ex) {
            Logger.getLogger(Creature.class.getName()).log(Level.SEVERE, null, ex);
        }
        
        return c;
    }

    @Override
    public Object clone() throws CloneNotSupportedException 
    { 
        return super.clone(); 
    } 
}
