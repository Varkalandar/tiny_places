package tinyplaces.server.data;

/**
 * Damage types and calculations.
 * 
 * @author hjm
 */
public class Damage 
{
    public static int TYPE_PHYSICAL = 0;
    public static int TYPE_FIRE = 1;
    public static int TYPE_COLD = 2;
    public static int TYPE_LIGHT = 3;
    public static int TYPE_CHAOS = 4;
    public static int TYPE_COUNT = 5;
    
    public static int calculate(Creature target, Spell attack)
    {
        int damage[] = new int[TYPE_COUNT];
        
        // plain damage
        for(int i=0; i<TYPE_COUNT; i++)
        {
            damage[i] = attack.min[i] + (int)((attack.max[i] - attack.min[i] + 1) * Math.random());
        }
        
        // resistances -> damage afterwards is in percent!
        for(int i=0; i<TYPE_COUNT; i++)
        {
            damage[i] = damage[i] * (100 - target.resistance[i]);
        }
        
        int total = 0;

        for(int i=0; i<TYPE_COUNT; i++)
        {
            total += damage[i];
        }
        
        return total / 100;
    }
}
