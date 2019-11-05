package org.rustwallet.android.gson;

import com.google.gson.TypeAdapter;
import com.google.gson.stream.JsonReader;
import com.google.gson.stream.JsonToken;
import com.google.gson.stream.JsonWriter;

import java.io.IOException;
import java.util.Date;

public class DateTypeAdapter extends TypeAdapter<Date> {

    @Override
    public void write(JsonWriter writer, Date date) throws IOException {
        if (date == null) {
            writer.nullValue();
            return;
        }
        writer.value(date.getTime());
    }

    @Override
    public Date read(JsonReader reader) throws IOException {
        if (reader.peek() == JsonToken.NULL) {
            reader.nextNull();
            return null;
        }
        Long date = reader.nextLong();
        return new Date(date*1000);
    }
}
