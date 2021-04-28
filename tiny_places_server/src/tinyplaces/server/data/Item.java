package tinyplaces.server.data;

import java.awt.Point;

/**
 * An actual item in the game. The base item carries all the fixed
 * stats, and this class contains all the variable parts.
 * 
 * @author hjm
 */
public class Item 
{
    public static int ON_MAP = -1;
    public static int IN_INVENTORY = -2;
    public static int IN_FIRST_SLOT = 0;

    public final BaseItem baseItem;
    
    public int id;
    public String displayName;
    
    /** Map, inventory, equipment slot. Slots have positive numbers */
    public int where = ON_MAP;
    
    /** 
     * If the place given in "where" supports more than 1
     * item position, the current position is given here. E.g.
     * the exact location in the inventory.
     */
    public final Point position;
    
    public final int energyDamage;
    public final int physicalDamage;
    
    public Item(BaseItem baseItem)
    {
        this.baseItem = baseItem;
        this.position = new Point();
        energyDamage = randomFromRange(baseItem.energyDamageMin, baseItem.energyDamageMax);
        physicalDamage = randomFromRange(baseItem.physicalDamageMin, baseItem.physicalDamageMax);
    }

    private int randomFromRange(float min, float max) 
    {
        return (int)(min + Math.random() * (max-min+1));
    }
}
