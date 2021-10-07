import { ApolloProvider, ApolloClient, ApolloLink, createHttpLink, FetchResult, InMemoryCache, Operation, Observable, split } from "@apollo/client"
import { setContext } from '@apollo/client/link/context'
import { getMainDefinition } from "@apollo/client/utilities"
import { onError } from "@apollo/client/link/error"
import { GraphQLError, print } from "graphql"
import { Client, ClientOptions, createClient } from "graphql-ws"
import React, { PropsWithChildren } from "react"
import { RouteComponentProps, withRouter } from "react-router"

class WebSocketLink extends ApolloLink {
    private client: Client

    constructor(options: ClientOptions) {
        super()
        this.client = createClient(options)
    }

    public request(operation: Operation): Observable<FetchResult> {
        return new Observable((sink) => {
            return this.client.subscribe<FetchResult>(
                { ...operation, query: print(operation.query) },
                {
                    next: sink.next.bind(sink),
                    complete: sink.complete.bind(sink),
                    error: (err) => {
                        if (err instanceof Error) {
                            return sink.error(err)
                        }

                        if (err instanceof CloseEvent) {
                            return sink.error(
                                new Error(
                                    `Socket closed with event ${err.code} ${err.reason || ''}`,
                                ),
                            )
                        }

                        return sink.error(
                            new Error(
                                (err as GraphQLError[])
                                    .map(({ message }) => message)
                                    .join(', '),
                            ),
                        )
                    },
                },
            )
        })
    }
}

class IncludeApolloClass extends React.Component<PropsWithChildren<RouteComponentProps>> {

    subscriptionLink = new WebSocketLink({
        url: 'ws://localhost:8000/graphql',
        connectionParams: () => {
            const token = localStorage.getItem('token')
            if (!token) {
                return {}
            }
            return {
                authorization: token ?? "",
            }
        },
    })

    httpLink = createHttpLink({
        uri: 'http://localhost:8000/graphql',
    })

    authLink = setContext((_, { headers }) => {
        const token = localStorage.getItem('token')
        return {
            headers: {
                ...headers,
                authorization: token ?? "",
            }
        }
    })

    splitLink = split(
        ({ query }) => {
            const definition = getMainDefinition(query)
            return (
                definition.kind === 'OperationDefinition' &&
                definition.operation === 'subscription'
            )
        },
        this.subscriptionLink,
        this.httpLink,
    )

    errorLink = onError(({ graphQLErrors, networkError }) => {
        if (graphQLErrors)
            graphQLErrors.forEach(({ message, locations, path }) => {
                if (message === "Not logged in.") {
                    this.props.history.push(`/login?${new URLSearchParams({ "redirect-to": this.props.location.pathname }).toString()}`)
                } else {
                    console.log(`[GraphQL error]: Message: ${message}, Location: ${locations}, Path: ${path}`)
                }
            })

        if (networkError) console.log(`[Network error]: ${networkError}`)
    })

    client = new ApolloClient({
        link: this.errorLink.concat(this.authLink).concat(this.splitLink),
        defaultOptions: {
            query: {
                errorPolicy: "all"
            }
        },
        cache: new InMemoryCache()
    })

    render() {
        return (
            <ApolloProvider client={this.client}>
                {this.props.children}
            </ApolloProvider>
        )
    }
}

export const IncludeApollo = withRouter(IncludeApolloClass)