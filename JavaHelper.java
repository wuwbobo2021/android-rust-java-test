import android.app.Activity;

public class JavaHelper {
    Activity mActivity;

    public JavaHelper(Activity activity) {
        this.mActivity = activity;
        this.mActivity.runOnUiThread(new Runnable() {
            @Override
            public void run() {}
        });
    }

    public String test_get_string() {
        return "String from Java!";
    }
}
