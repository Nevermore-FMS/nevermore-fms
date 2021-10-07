import { useState } from "react"
import { useSignInMutation } from "../../generated/graphql"
import styles from "./index.module.scss"
import TextField from "../../styles/ohms-style/react/components/TextField"
import Button from "../../styles/ohms-style/react/components/Button"
import { useHistory } from "react-router"
import Roboticon2021Header from "../../components/Roboticon2021Header"

export default function LoginPage() {
    const history = useHistory()
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")
    const [signIn, { data, loading, error }] = useSignInMutation({ errorPolicy: "all" })

    if (data?.signIn != null) {
        localStorage.setItem('token', `Bearer ${data.signIn}`)
        const params = new URLSearchParams(history.location.search)
        if (params.get("redirect-to") != null) {
            history.push(params.get("redirect-to")!)
        } else {
            history.push("/")
        }
    }

    return (
        <div>
            <Roboticon2021Header />
            <div className="container">
                <h1>Login</h1>
                {error?.graphQLErrors.map((e, i) => (
                    <div key={i} className="error">
                        {e.message}
                    </div>
                ))}
                <div className={styles.form}>
                    <TextField placeholder="Username" value={username} onChange={(e) => setUsername(e.target.value)} />
                    <TextField password placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
                    <Button disabled={loading} onClick={() => {
                        signIn({
                            variables: {
                                username,
                                password
                            }
                        })
                    }}>Login</Button>
                </div>
            </div>
        </div>
    )
}