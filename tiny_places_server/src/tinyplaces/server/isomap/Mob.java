package tinyplaces.server.isomap;

import tinyplaces.server.data.Creature;
import tinyplaces.server.data.Spell;

/**
 * Any type of map object
 * @author hjm
 */
public class Mob 
{
    public static int TYPE_PROP = 0;
    public static int TYPE_CREATURE = 1;
    public static int TYPE_PLAYER = 2;
    public static int TYPE_PROJECTILE = 3;
    public int id;
    public int tile;
    public int x;
    public int y;
    public float scale;
    public String color;
    public int type;
    public long nextAiTime;
    public Creature creature; // Usually only set if this is TYPE_CREATURE
    public Spell spell; // Usually only set if this is TYPE_PROJECTILE
    
    @Override
    public boolean equals(Object o) {
        if(o instanceof Mob)
        {
            Mob other = (Mob)o;
            return other.id == id;
        }
        
        return false;
    }

    @Override
    public int hashCode() {
        int hash = 3;
        hash = 67 * hash + this.id;
        return hash;
    }
}
