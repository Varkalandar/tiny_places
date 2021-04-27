package tinyplaces.server.data;

/**
 *
 * @author hjm
 */
public class ItemBuilder 
{
    public static Item create(String baseId)
    {
        BaseItem baseItem = ItemCatalog.get(baseId);
        
        Item item = new Item(baseItem);
        item.displayName = baseItem.displayName;
        
        
        
        // todo - randomly enhanced items
        // todo - random names for special items
        
        return item;
    }
}
