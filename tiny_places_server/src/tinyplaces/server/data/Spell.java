package tinyplaces.server.data;

/**
 * A set of spell data.
 * 
 * @author hjm
 */
public class Spell 
{
    String id;
    public String displayName;
    public String ptype;
    
    int min[] = new int [Damage.TYPE_COUNT];
    int max[] = new int [Damage.TYPE_COUNT];
    public int speed;
    public int castTime; // milliseconds
}
