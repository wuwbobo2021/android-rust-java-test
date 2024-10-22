import android.app.Activity;

public class JavaHelper {
    int counter;
    Activity mActivity;

    public JavaHelper(Activity activity) {
        this.counter = 0;
        this.mActivity = activity;
        this.mActivity.runOnUiThread(new Runnable() {
            @Override
            public void run() {}
        });
    }

    public String test_get_string() {
        this.counter += 1;
        return "String from Java! " + this.counter;
    }
}
