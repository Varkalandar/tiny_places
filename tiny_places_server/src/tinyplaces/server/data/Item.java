package tinyplaces.server.data;

/**
 * An actual item in the game. The base item carries all the fixed
 * stats, and this class contains all the variable parts.
 * 
 * @author hjm
 */
public class Item 
{
    public static int ON_MAP = 0;
    public static int IN_INVENTORY = 1;
    
    public int where = ON_MAP;
    
    public final BaseItem baseItem;
    
    public Item(BaseItem baseItem)
    {
        this.baseItem = baseItem;
    }
    
    
    
}
