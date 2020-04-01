package tinyplaces.server.isomap.actions;

import tinyplaces.server.isomap.Room;

/**
 * Interface for all ongoing actions that happen in a room.
 * 
 * @author hjm
 */
public interface MapAction 
{
    public void process(Room room, int dt);

    public boolean isDone();
}
