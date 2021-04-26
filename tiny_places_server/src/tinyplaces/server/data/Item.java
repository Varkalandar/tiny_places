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
    
    /** Map, inventory, equipment slot. Slots have positive numbers */
    public int where = ON_MAP;

    public int id;
    
    /** 
     * If the place given in "where" supports more than 1
     * item position, the current position is given here. E.g.
     * the exact location in the inventory.
     */
    public final Point position;
    
    public final BaseItem baseItem;
    
    public Item(BaseItem baseItem)
    {
        this.baseItem = baseItem;
        this.position = new Point();
    }
}
