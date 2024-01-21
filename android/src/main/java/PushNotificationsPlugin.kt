package ru.clubgermes.social.plugin_firebase

import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

import com.google.firebase.messaging.FirebaseMessaging
import com.google.android.gms.tasks.OnCompleteListener
import android.util.Log
import app.tauri.plugin.Channel

import com.google.firebase.Firebase
import com.google.firebase.messaging.messaging

@InvokeArg
class PingArgs {
    var value: String? = null
}

@InvokeArg
class RegisterPushNotificationHandlerArgs {
    lateinit var handler: Channel
}

@TauriPlugin
class PushNotificationsPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun registerPushNotificationHandler(invoke: Invoke) {
        Log.w("registerPushNHandler", "")
        val args = invoke.parseArgs(RegisterPushNotificationHandlerArgs::class.java)
        PushNotificationsPlugin.channel = args.handler
        invoke.resolve()
    }

    // [START что-то сам]
    @Command
    fun getMyToken(invoke: Invoke) {
        Log.e("getMyToken= ", "" + myToken)
        val ret = JSObject()
        ret.put("myToken", myToken)
        invoke.resolve(ret)
    }
    // [END что-то сам]

    override fun load(webView: WebView) {
        FirebaseMessaging.getInstance().token.addOnCompleteListener(OnCompleteListener { task ->
            if (task.isSuccessful) {
                // Get new FCM registration token
                val token = task.result
                Log.e("myToken1", "" + token)
            } else {
                Log.w("Fetching token failed", task.exception)
            }
        })

        // Get token
        // [START log_reg_token]
        Firebase.messaging.token.addOnCompleteListener(
            OnCompleteListener { task ->
                if (!task.isSuccessful) {
                    Log.w(TAG, "Fetching FCM registration token failed", task.exception)
                    return@OnCompleteListener
                }

                // Get new FCM registration token
                val token = task.result
                myToken = token

                // Log and toast
//                val msg = getString(R.string.msg_token_fmt, token)
//                Log.d(TAG, msg)
//                Toast.makeText(baseContext, msg, Toast.LENGTH_SHORT).show()
                Log.e("myToken2", "" + token)
            },
        )
        // [END log_reg_token]

    }

    companion object {

        private const val TAG = "Tauri plugin load"
        var channel: Channel? = null
        var myToken = ""
    }
}
