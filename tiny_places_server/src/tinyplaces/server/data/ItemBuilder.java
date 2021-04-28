package tinyplaces.server.data;

/**
 *
 * @author hjm
 */
public class ItemBuilder 
{
    // Todo - make this a permanent counter
    // over server restarts. There should never
    // be duplicate item id's
    private static int nextID = 1;
    
    public static Item create(String baseId)
    {
        BaseItem baseItem = ItemCatalog.get(baseId);
        
        Item item = new Item(baseItem);
        item.displayName = baseItem.displayName;
        item.id = nextID ++;
        
        
        // todo - randomly enhanced items
        // todo - random names for special items
        
        return item;
    }
}
