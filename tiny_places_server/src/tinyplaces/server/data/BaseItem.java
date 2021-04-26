package tinyplaces.server.data;

/**
 *
 * @author hjm
 */
public class BaseItem 
{

    public String id;
    public String displayName;
    public int tile;
    public int baseValue;
    public int[] resistance = new int [Damage.TYPE_COUNT];
    public String color;
    public float scale;
    
}
