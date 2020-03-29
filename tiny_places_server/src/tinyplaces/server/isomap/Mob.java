package tinyplaces.server.isomap;

/**
 * Any type of map object
 * @author hjm
 */
public class Mob 
{
    public static int TYPE_PROP = 0;
    public static int TYPE_CREATURE = 1;
    public static int TYPE_PLAYER = 2;
    public int id;
    public int tile;
    public int x;
    public int y;
    public float scale;
    public String color;
    public int type;
    
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
