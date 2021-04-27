package tinyplaces.server.data;

/**
 * Values of this class should never be modified after creation.
 * 
 * @author hjm
 */
public class BaseItem 
{

    public String id;
    public String displayName;
    public int tile;
    public int width;
    public int height;
    public int baseValue;
    public int[] resistance = new int [Damage.TYPE_COUNT];
    public String color;
    public float scale;
    public float energyDamage;
    
}
