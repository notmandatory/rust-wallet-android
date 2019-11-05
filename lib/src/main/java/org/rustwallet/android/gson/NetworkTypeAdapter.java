package org.rustwallet.android.gson;

import com.google.gson.TypeAdapter;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonToken;
import com.google.gson.stream.JsonWriter;

import org.rustwallet.android.Network;

import java.io.IOException;

public class NetworkTypeAdapter extends TypeAdapter<Network> {

    @Override
    public void write(JsonWriter writer, Network network) throws IOException {
        if (network == null) {
            writer.nullValue();
            return;
        }
        writer.value(network.toString().toLowerCase());
    }

    @Override
    public Network read(JsonReader reader) throws IOException {
        if (reader.peek() == JsonToken.NULL) {
            reader.nextNull();
            return null;
        }
        String name = reader.nextString();
        return Network.valueOf(name.substring(0, 1).toUpperCase() + name.substring(1).toLowerCase());
    }
}
