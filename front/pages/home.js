import { useAuth0 } from "@auth0/auth0-react";
import React from "react";

const HomePage = () => {
  const { isAuthenticated, logout } = useAuth0();

  return (
    <div>
      <h2>ログイン状態</h2>
      {isAuthenticated ? (
          <>
            <p>ログイン中です</p>
            <button onClick={() => logout({ returnTo: window.location.origin })}>
              ログアウト
            </button>
          </>
        ) : (
          <p>ログアウトしています</p>
        )}
    </div>
  );
};

export default HomePage;
