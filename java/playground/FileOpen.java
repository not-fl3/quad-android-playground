package playground;

import android.content.Intent;
import android.content.IntentFilter;
import android.util.Log;
import android.net.Uri;
import android.os.ParcelFileDescriptor;
import android.content.Context;

import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.IOException;

import TARGET_PACKAGE_NAME.MainActivity;

public class FileOpen {
    public static MainActivity MainActivity;
    public native static void FileOpenInit();
    native static void FileOpenOnReceive(byte[] data);
    
    public FileOpen() {
    }

    public static void OnDataReceived(Context context, Uri file_path) {
        if (file_path != null) {
            byte[] bytes = null;
            try {
                bytes = readUri(context, file_path);
                FileOpenOnReceive(bytes);
            } catch (IOException e) {
                Log.w("SAPP", e.toString());
            }
        }
    }

    private static byte[] readUri(Context context, Uri uri) throws IOException {
        ParcelFileDescriptor file = context.getContentResolver().openFileDescriptor(uri, "r");

        assert file != null;
        assert file.getStatSize() <= Integer.MAX_VALUE;
        byte[] data = new byte[(int) file.getStatSize()];
        
        FileDescriptor fd = file.getFileDescriptor();
        FileInputStream fileStream = new FileInputStream(fd);
        fileStream.read(data);
        
        return data;
    }


    public void OpenFileDialog() {
        MainActivity.OpenFileDialog();
    }
}
