import { useState } from "react"
import { SignInDocument, SignInMutation, SignInMutationVariables } from "../../generated/graphql"
import styles from "./index.module.scss"
import TextField from "../../styles/ohms-style/react/components/TextField"
import Button from "../../styles/ohms-style/react/components/Button"
import { useHistory } from "react-router"
import Roboticon2021Header from "../../components/Roboticon2021Header"
import { useApolloClient } from "@apollo/client"
import { GraphQLError } from "graphql"

export default function LoginPage() {
    const history = useHistory()
    const client = useApolloClient()
    const [error, setError] = useState<readonly GraphQLError[] | null>(null)
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")

    const signIn = async () => {
        const result = await client.mutate<SignInMutation, SignInMutationVariables>({
            mutation: SignInDocument,
            variables: {
                username, password
            },
            errorPolicy: "all"
        })
        if (result.errors) {
            setError(result.errors)
            return
        }
        localStorage.setItem('token', `Bearer ${result.data?.signIn}`)
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
                {error?.map((e, i) => (
                    <div key={i} className="error">
                        {e.message}
                    </div>
                ))}
                <div className={styles.form}>
                    <TextField placeholder="Username" value={username} onChange={(e) => setUsername(e.target.value)} />
                    <TextField password placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} />
                    <Button onClick={() => signIn()}>Login</Button>
                </div>
            </div>
        </div>
    )
}