package tinyplaces.server.data;

/**
 * Values of this class should never be modified after creation.
 * 
 * @author hjm
 */
public class BaseItem 
{
    public static String CLASS_CORE = "core";
    public static String CLASS_POWERUP = "powerup";
    public static String TYPE_FUNCTION = "func";

    public String id;
    public String displayName;
    public String iclass;
    public String itype;        
    public int tile;
    public int shadow;         // shadow tile number
    public float shadowScale;  // shadow scale factor
    public int width;
    public int height;
    public boolean canDrop;
    public int stackSize;
    public int baseValue;
    public int[] resistance = new int [Damage.TYPE_COUNT];
    public String color;
    public float scale;
    public int energyDamageMin;
    public int energyDamageMax;
    public int physicalDamageMin;
    public int physicalDamageMax;
    
    public String description;
}
