import { initializeApp } from "firebase/app";
import {
  browserLocalPersistence,
  getAuth,
  onIdTokenChanged,
  setPersistence,
  signInWithEmailAndPassword,
  signOut,
  type User
} from "firebase/auth";

const firebaseConfig = {
  apiKey: import.meta.env.VITE_FIREBASE_API_KEY,
  authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN,
  projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID,
  appId: import.meta.env.VITE_FIREBASE_APP_ID
};

export const hasFirebaseConfig = Object.values(firebaseConfig).every(Boolean);

const app = hasFirebaseConfig ? initializeApp(firebaseConfig) : null;
export const auth = app ? getAuth(app) : null;

if (auth) {
  void setPersistence(auth, browserLocalPersistence);
}

export function watchToken(callback: (user: User | null, token: string) => void) {
  if (!auth) {
    callback(null, "");
    return () => {};
  }

  return onIdTokenChanged(auth, async (user) => {
    callback(user, user ? await user.getIdToken() : "");
  });
}

export async function signIn(email: string, password: string) {
  if (!auth) {
    throw new Error("Firebase web config is missing");
  }

  const credential = await signInWithEmailAndPassword(auth, email, password);
  return credential.user.getIdToken(true);
}

export async function signOutAdmin() {
  if (auth) {
    await signOut(auth);
  }
}
