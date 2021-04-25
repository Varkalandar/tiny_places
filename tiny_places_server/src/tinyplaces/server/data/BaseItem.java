package tinyplaces.server.data;

/**
 *
 * @author hjm
 */
public class BaseItem 
{

    String id;
    String displayName;
    int tile;
    int[] resistance = new int [Damage.TYPE_COUNT];
    String color;
    float scale;
    
}
