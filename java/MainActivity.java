//% IMPORTS

import android.app.Activity;

import playground.FileOpen;

//% END

//% MAIN_ACTIVITY_BODY

private static final int FILE_REQUEST_CODE = 12;

public void OpenFileDialog() { 
    Intent myIntent = new Intent(Intent.ACTION_GET_CONTENT, null);
    myIntent.setType("image/png");
    startActivityForResult(myIntent, FILE_REQUEST_CODE);
}
//% END

//% MAIN_ACTIVITY_ON_ACTIVITY_RESULT

if (requestCode == FILE_REQUEST_CODE) {
    if (resultCode == Activity.RESULT_OK) {
        Log.w("SAPP", "URI: " + data.getData());
        FileOpen.OnDataReceived(this, data.getData());
    } else {
        FileOpen.OnDataReceived(this, null);
    }
}
//% END



//% MAIN_ACTIVITY_ON_CREATE

FileOpen.MainActivity = this;
FileOpen.FileOpenInit();

//% END

