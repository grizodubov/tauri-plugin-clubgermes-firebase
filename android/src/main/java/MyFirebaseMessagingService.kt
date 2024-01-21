package ru.clubgermes.social.plugin_firebase

import android.util.Log
import app.tauri.plugin.JSObject
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage

class MyFirebaseMessagingService : FirebaseMessagingService() {

    // [START on_new_token]
    /**
     * Called if the FCM registration token is updated. This may occur if the security of
     * the previous token had been compromised. Note that this is called when the
     * FCM registration token is initially generated so this is where you would retrieve the token.
     */
    override fun onNewToken(token: String) {
        Log.d(TAG, "Refreshed token: $token")

        // If you want to send messages to this application instance or
        // manage this apps subscriptions on the server side, send the
        // FCM registration token to your app server.
        sendRegistrationToServer(token)
    }
    // [END on_new_token]

    /**
     * Persist token to third-party servers.
     *
     * Modify this method to associate the user's FCM registration token with any server-side account
     * maintained by your application.
     *
     * @param token The new token.
     */
    private fun sendRegistrationToServer(token: String?) {
        // TODO: Implement this method to send token to your app server.
        Log.d(TAG, "sendRegistrationTokenToServer($token)")
    }

    override fun onMessageReceived(message: RemoteMessage) {
        super.onMessageReceived(message)
        Log.i("MyFCMService ", "Message :: $message")

        val data = JSObject()

        for (entry in message.data.entries.iterator()) {
            data.put(entry.key, entry.value)
        }

        val message = JSObject()
        message.put("data", data)

        var tries = 0;

        if (PushNotificationsPlugin.channel == null) {
            runn()

            while (PushNotificationsPlugin.channel == null && tries < 60) {
                Thread.sleep(500)  // wait for 1 second
                tries += 1

                Log.e("channel is ", "null")
            }
        }

        PushNotificationsPlugin.channel?.send(message)
    }

    private external fun runn()

    companion object {

        private const val TAG = "MyFirebaseMsgService"

        init {
            System.loadLibrary("tauri_app_lib")
        }
    }
}