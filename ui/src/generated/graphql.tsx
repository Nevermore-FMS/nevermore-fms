import { gql } from '@apollo/client';
import * as Apollo from '@apollo/client';
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
const defaultOptions =  {}
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

/**
 * Represents the AllianceStation of a DriverStation. There are six different alliance stations around
 * an FRC field, three on each side. (Hardcoded due to it's use in the network protocol)
 */
export enum AllianceStation {
  Blue_1 = 'BLUE_1',
  Blue_2 = 'BLUE_2',
  Blue_3 = 'BLUE_3',
  None = 'NONE',
  Red_1 = 'RED_1',
  Red_2 = 'RED_2',
  Red_3 = 'RED_3'
}

export type AllianceStationConfiguration = {
  password: Scalars['String'];
  ssid: Scalars['String'];
};

export type AllianceStationToConfiguration = {
  blue1: AllianceStationConfiguration;
  blue2: AllianceStationConfiguration;
  blue3: AllianceStationConfiguration;
  red1: AllianceStationConfiguration;
  red2: AllianceStationConfiguration;
  red3: AllianceStationConfiguration;
};

export enum ConfigKey {
  ActiveNetworkConfigurator = 'ACTIVE_NETWORK_CONFIGURATOR',
  EventName = 'EVENT_NAME',
  HasSetup = 'HAS_SETUP',
  PrivateKey = 'PRIVATE_KEY',
  ShareCrashAnalytics = 'SHARE_CRASH_ANALYTICS'
}

export type ConfirmedState = {
  __typename?: 'ConfirmedState';
  batteryVoltage: Scalars['Float'];
  canPingRadio: Scalars['Boolean'];
  canPingRio: Scalars['Boolean'];
  isEmergencyStopped: Scalars['Boolean'];
  isEnabled: Scalars['Boolean'];
  mode: Mode;
  robotCommunicationsActive: Scalars['Boolean'];
  teamNumber: Scalars['Int'];
};

export type CreatePluginParams = {
  author: Scalars['String'];
  code: Scalars['String'];
  email: Scalars['String'];
  enabled: Scalars['Boolean'];
  frontendCode: Scalars['String'];
  hasFrontend: Scalars['Boolean'];
  name: Scalars['String'];
  pluginType: PluginType;
  readme: Scalars['String'];
  url: Scalars['String'];
};

export type CreateUserParams = {
  fullName: Scalars['String'];
  password: Scalars['String'];
  pin: Scalars['String'];
  userType: UserType;
  username: Scalars['String'];
};

/**
 * Represents the Status of a Driverstation. Used to tell the operator of a Driverstation
 * whether they should be in a game and whether they're in the correct station. Send
 * `DriverstationStatus::Good` when in the correct position, `DriverstationStatus::Bad`
 * when in the wrong position, and `DriverstationStatus::Waiting` when the team isn't in
 * this match.
 */
export enum DriverstationStatus {
  Bad = 'BAD',
  Good = 'GOOD',
  Waiting = 'WAITING'
}

export type LogMessage = {
  __typename?: 'LogMessage';
  dateTime: Scalars['String'];
  level: Scalars['Int'];
  message: Scalars['String'];
};

/**
 * Represents the Mode of a DriverStation. These values correspond to the values you can
 * get from WPILib and can set on the Driverstation when directly connected.
 */
export enum Mode {
  Autonomous = 'AUTONOMOUS',
  TeleOp = 'TELE_OP',
  Test = 'TEST'
}

export type Mutation = {
  __typename?: 'Mutation';
  addTeamToField: Scalars['Boolean'];
  deletePlugin: Scalars['Boolean'];
  deleteUser: Scalars['Boolean'];
  devCreatePlugin: Scalars['Boolean'];
  devDeletePlugin: Scalars['Boolean'];
  devRestartPlugin: Scalars['Boolean'];
  networkInitialConfiguration: Scalars['Boolean'];
  networkMatchConfiguration: Scalars['Boolean'];
  networkScan: Scalars['Boolean'];
  publish: Scalars['Boolean'];
  removeTeamFromField: Scalars['Boolean'];
  setAllTeamsEmergencyStopped: Scalars['Boolean'];
  setAllTeamsEnabled: Scalars['Boolean'];
  setTeamState: Scalars['Boolean'];
  setupUpsertUser: Scalars['Boolean'];
  signIn: Scalars['String'];
  signOut: Scalars['Boolean'];
  updateConfigEntry: Scalars['Boolean'];
  upsertPlugin: Scalars['Boolean'];
  upsertUser: Scalars['Boolean'];
};


export type MutationAddTeamToFieldArgs = {
  allianceStation: AllianceStation;
  teamNumber: Scalars['Int'];
};


export type MutationDeletePluginArgs = {
  name: Scalars['String'];
};


export type MutationDeleteUserArgs = {
  username: Scalars['String'];
};


export type MutationDevCreatePluginArgs = {
  params: CreatePluginParams;
};


export type MutationDevDeletePluginArgs = {
  name: Scalars['String'];
};


export type MutationNetworkMatchConfigurationArgs = {
  stationConfig: AllianceStationToConfiguration;
};


export type MutationPublishArgs = {
  message: Scalars['String'];
  topic: Scalars['String'];
};


export type MutationRemoveTeamFromFieldArgs = {
  teamNumber: Scalars['Int'];
};


export type MutationSetAllTeamsEmergencyStoppedArgs = {
  emergencyStop: Scalars['Boolean'];
};


export type MutationSetAllTeamsEnabledArgs = {
  enabled: Scalars['Boolean'];
};


export type MutationSetTeamStateArgs = {
  stateInput: StateInput;
  teamNumber: Scalars['Int'];
};


export type MutationSetupUpsertUserArgs = {
  params: CreateUserParams;
};


export type MutationSignInArgs = {
  password: Scalars['String'];
  username: Scalars['String'];
};


export type MutationUpdateConfigEntryArgs = {
  key: ConfigKey;
  value: Scalars['String'];
};


export type MutationUpsertPluginArgs = {
  params: CreatePluginParams;
};


export type MutationUpsertUserArgs = {
  params: CreateUserParams;
};

export type NetworkConfiguratorInfo = {
  __typename?: 'NetworkConfiguratorInfo';
  author: Scalars['String'];
  email: Scalars['String'];
  name: Scalars['String'];
  readme: Scalars['String'];
  supportedHardware: Array<Scalars['String']>;
  timeout: Scalars['Int'];
  url: Scalars['String'];
};

export type Node = {
  id: Scalars['ID'];
};

/** Information about pagination in a connection */
export type PageInfo = {
  __typename?: 'PageInfo';
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']>;
};

export type Plugin = Node & {
  __typename?: 'Plugin';
  author: Scalars['String'];
  code: Scalars['String'];
  email: Scalars['String'];
  enabled: Scalars['Boolean'];
  frontendCode: Scalars['String'];
  hasFrontend: Scalars['Boolean'];
  id: Scalars['ID'];
  name: Scalars['String'];
  pluginType: PluginType;
  readme: Scalars['String'];
  url: Scalars['String'];
};

export type PluginConnection = {
  __typename?: 'PluginConnection';
  /** A list of edges. */
  edges?: Maybe<Array<Maybe<PluginEdge>>>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type PluginEdge = {
  __typename?: 'PluginEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: Plugin;
};

export enum PluginType {
  Game = 'GAME',
  Generic = 'GENERIC',
  NetworkConfigurator = 'NETWORK_CONFIGURATOR'
}

export type Query = {
  __typename?: 'Query';
  configEntry?: Maybe<Scalars['String']>;
  connectedTeamNumbers: Array<Scalars['Int']>;
  devPlugins: Array<Plugin>;
  networkConfiguratorAllInfo: Array<NetworkConfiguratorInfo>;
  networkConfiguratorInfo: NetworkConfiguratorInfo;
  node: Node;
  plugins: PluginConnection;
  robotConfirmedState: ConfirmedState;
  robotState: State;
  teamAllianceStations: Array<TeamAllianceStation>;
  users: UserConnection;
};


export type QueryConfigEntryArgs = {
  key: ConfigKey;
};


export type QueryNetworkConfiguratorInfoArgs = {
  name: Scalars['String'];
};


export type QueryNodeArgs = {
  id: Scalars['ID'];
};


export type QueryPluginsArgs = {
  after?: Maybe<Scalars['String']>;
  before?: Maybe<Scalars['String']>;
  first?: Maybe<Scalars['Int']>;
  last?: Maybe<Scalars['Int']>;
};


export type QueryRobotConfirmedStateArgs = {
  teamNumber: Scalars['Int'];
};


export type QueryRobotStateArgs = {
  teamNumber: Scalars['Int'];
};


export type QueryUsersArgs = {
  after?: Maybe<Scalars['String']>;
  before?: Maybe<Scalars['String']>;
  first?: Maybe<Scalars['Int']>;
  last?: Maybe<Scalars['Int']>;
};

export type State = {
  __typename?: 'State';
  allianceStation: AllianceStation;
  emergencyStop: Scalars['Boolean'];
  enable: Scalars['Boolean'];
  eventName: Scalars['String'];
  matchNumber: Scalars['Int'];
  mode: Mode;
  sequenceNumber: Scalars['Int'];
  status: DriverstationStatus;
  teamNumber: Scalars['Int'];
  timeToDisplay: Scalars['Int'];
};

export type StateInput = {
  allianceStation: AllianceStation;
  emergencyStop: Scalars['Boolean'];
  enable: Scalars['Boolean'];
  eventName: Scalars['String'];
  matchNumber: Scalars['Int'];
  mode: Mode;
  sequenceNumber: Scalars['Int'];
  status: DriverstationStatus;
  timeToDisplay: Scalars['Int'];
};

export type Subscription = {
  __typename?: 'Subscription';
  devLog: LogMessage;
  fieldClose: Scalars['Boolean'];
  fieldTick: Scalars['Boolean'];
  subscribe: Scalars['String'];
};


export type SubscriptionSubscribeArgs = {
  topic: Scalars['String'];
};

export type TeamAllianceStation = {
  __typename?: 'TeamAllianceStation';
  allianceStation: AllianceStation;
  teamNumber: Scalars['Int'];
};

export type User = Node & {
  __typename?: 'User';
  fullName: Scalars['String'];
  id: Scalars['ID'];
  userType: UserType;
  username: Scalars['String'];
};

export type UserConnection = {
  __typename?: 'UserConnection';
  /** A list of edges. */
  edges?: Maybe<Array<Maybe<UserEdge>>>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
};

/** An edge in a connection. */
export type UserEdge = {
  __typename?: 'UserEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: User;
};

export enum UserType {
  Admin = 'ADMIN',
  Referee = 'REFEREE',
  Viewer = 'VIEWER'
}

export type AddTeamToFieldMutationVariables = Exact<{
  teamNumber: Scalars['Int'];
  allianceStation: AllianceStation;
}>;


export type AddTeamToFieldMutation = { __typename?: 'Mutation', addTeamToField: boolean };

export type EStopRoboticonGameMutationVariables = Exact<{
  eStop: Scalars['Boolean'];
}>;


export type EStopRoboticonGameMutation = { __typename?: 'Mutation', publish: boolean };

export type GetTeamAllianceStationsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTeamAllianceStationsQuery = { __typename?: 'Query', teamAllianceStations: Array<{ __typename?: 'TeamAllianceStation', teamNumber: number, allianceStation: AllianceStation }> };

export type RemoveTeamFromFieldMutationVariables = Exact<{
  teamNumber: Scalars['Int'];
}>;


export type RemoveTeamFromFieldMutation = { __typename?: 'Mutation', removeTeamFromField: boolean };

export type RequestRoboticonScoresMutationVariables = Exact<{ [key: string]: never; }>;


export type RequestRoboticonScoresMutation = { __typename?: 'Mutation', publish: boolean };

export type RoboticonGameStateSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type RoboticonGameStateSubscription = { __typename?: 'Subscription', subscribe: string };

export type RoboticonScoresSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type RoboticonScoresSubscription = { __typename?: 'Subscription', subscribe: string };

export type SignInMutationVariables = Exact<{
  username: Scalars['String'];
  password: Scalars['String'];
}>;


export type SignInMutation = { __typename?: 'Mutation', signIn: string };

export type StartRoboticonGameMutationVariables = Exact<{ [key: string]: never; }>;


export type StartRoboticonGameMutation = { __typename?: 'Mutation', publish: boolean };

export type StopRoboticonGameMutationVariables = Exact<{ [key: string]: never; }>;


export type StopRoboticonGameMutation = { __typename?: 'Mutation', publish: boolean };

export type SwitchRoboticonGameMutationVariables = Exact<{
  game: Scalars['String'];
}>;


export type SwitchRoboticonGameMutation = { __typename?: 'Mutation', publish: boolean };

export type UpdateRoboticonScoreMutationVariables = Exact<{
  data: Scalars['String'];
}>;


export type UpdateRoboticonScoreMutation = { __typename?: 'Mutation', publish: boolean };


export const AddTeamToFieldDocument = gql`
    mutation AddTeamToField($teamNumber: Int!, $allianceStation: AllianceStation!) {
  addTeamToField(teamNumber: $teamNumber, allianceStation: $allianceStation)
}
    `;
export type AddTeamToFieldMutationFn = Apollo.MutationFunction<AddTeamToFieldMutation, AddTeamToFieldMutationVariables>;

/**
 * __useAddTeamToFieldMutation__
 *
 * To run a mutation, you first call `useAddTeamToFieldMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useAddTeamToFieldMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [addTeamToFieldMutation, { data, loading, error }] = useAddTeamToFieldMutation({
 *   variables: {
 *      teamNumber: // value for 'teamNumber'
 *      allianceStation: // value for 'allianceStation'
 *   },
 * });
 */
export function useAddTeamToFieldMutation(baseOptions?: Apollo.MutationHookOptions<AddTeamToFieldMutation, AddTeamToFieldMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<AddTeamToFieldMutation, AddTeamToFieldMutationVariables>(AddTeamToFieldDocument, options);
      }
export type AddTeamToFieldMutationHookResult = ReturnType<typeof useAddTeamToFieldMutation>;
export type AddTeamToFieldMutationResult = Apollo.MutationResult<AddTeamToFieldMutation>;
export type AddTeamToFieldMutationOptions = Apollo.BaseMutationOptions<AddTeamToFieldMutation, AddTeamToFieldMutationVariables>;
export const EStopRoboticonGameDocument = gql`
    mutation EStopRoboticonGame($eStop: Boolean!) {
  publish(topic: "roboticonSetAllEStopped", message: "true")
}
    `;
export type EStopRoboticonGameMutationFn = Apollo.MutationFunction<EStopRoboticonGameMutation, EStopRoboticonGameMutationVariables>;

/**
 * __useEStopRoboticonGameMutation__
 *
 * To run a mutation, you first call `useEStopRoboticonGameMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useEStopRoboticonGameMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [eStopRoboticonGameMutation, { data, loading, error }] = useEStopRoboticonGameMutation({
 *   variables: {
 *      eStop: // value for 'eStop'
 *   },
 * });
 */
export function useEStopRoboticonGameMutation(baseOptions?: Apollo.MutationHookOptions<EStopRoboticonGameMutation, EStopRoboticonGameMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<EStopRoboticonGameMutation, EStopRoboticonGameMutationVariables>(EStopRoboticonGameDocument, options);
      }
export type EStopRoboticonGameMutationHookResult = ReturnType<typeof useEStopRoboticonGameMutation>;
export type EStopRoboticonGameMutationResult = Apollo.MutationResult<EStopRoboticonGameMutation>;
export type EStopRoboticonGameMutationOptions = Apollo.BaseMutationOptions<EStopRoboticonGameMutation, EStopRoboticonGameMutationVariables>;
export const GetTeamAllianceStationsDocument = gql`
    query GetTeamAllianceStations {
  teamAllianceStations {
    teamNumber
    allianceStation
  }
}
    `;

/**
 * __useGetTeamAllianceStationsQuery__
 *
 * To run a query within a React component, call `useGetTeamAllianceStationsQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetTeamAllianceStationsQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetTeamAllianceStationsQuery({
 *   variables: {
 *   },
 * });
 */
export function useGetTeamAllianceStationsQuery(baseOptions?: Apollo.QueryHookOptions<GetTeamAllianceStationsQuery, GetTeamAllianceStationsQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetTeamAllianceStationsQuery, GetTeamAllianceStationsQueryVariables>(GetTeamAllianceStationsDocument, options);
      }
export function useGetTeamAllianceStationsLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetTeamAllianceStationsQuery, GetTeamAllianceStationsQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetTeamAllianceStationsQuery, GetTeamAllianceStationsQueryVariables>(GetTeamAllianceStationsDocument, options);
        }
export type GetTeamAllianceStationsQueryHookResult = ReturnType<typeof useGetTeamAllianceStationsQuery>;
export type GetTeamAllianceStationsLazyQueryHookResult = ReturnType<typeof useGetTeamAllianceStationsLazyQuery>;
export type GetTeamAllianceStationsQueryResult = Apollo.QueryResult<GetTeamAllianceStationsQuery, GetTeamAllianceStationsQueryVariables>;
export const RemoveTeamFromFieldDocument = gql`
    mutation RemoveTeamFromField($teamNumber: Int!) {
  removeTeamFromField(teamNumber: $teamNumber)
}
    `;
export type RemoveTeamFromFieldMutationFn = Apollo.MutationFunction<RemoveTeamFromFieldMutation, RemoveTeamFromFieldMutationVariables>;

/**
 * __useRemoveTeamFromFieldMutation__
 *
 * To run a mutation, you first call `useRemoveTeamFromFieldMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useRemoveTeamFromFieldMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [removeTeamFromFieldMutation, { data, loading, error }] = useRemoveTeamFromFieldMutation({
 *   variables: {
 *      teamNumber: // value for 'teamNumber'
 *   },
 * });
 */
export function useRemoveTeamFromFieldMutation(baseOptions?: Apollo.MutationHookOptions<RemoveTeamFromFieldMutation, RemoveTeamFromFieldMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<RemoveTeamFromFieldMutation, RemoveTeamFromFieldMutationVariables>(RemoveTeamFromFieldDocument, options);
      }
export type RemoveTeamFromFieldMutationHookResult = ReturnType<typeof useRemoveTeamFromFieldMutation>;
export type RemoveTeamFromFieldMutationResult = Apollo.MutationResult<RemoveTeamFromFieldMutation>;
export type RemoveTeamFromFieldMutationOptions = Apollo.BaseMutationOptions<RemoveTeamFromFieldMutation, RemoveTeamFromFieldMutationVariables>;
export const RequestRoboticonScoresDocument = gql`
    mutation RequestRoboticonScores {
  publish(topic: "roboticonRequestScores", message: "true")
}
    `;
export type RequestRoboticonScoresMutationFn = Apollo.MutationFunction<RequestRoboticonScoresMutation, RequestRoboticonScoresMutationVariables>;

/**
 * __useRequestRoboticonScoresMutation__
 *
 * To run a mutation, you first call `useRequestRoboticonScoresMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useRequestRoboticonScoresMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [requestRoboticonScoresMutation, { data, loading, error }] = useRequestRoboticonScoresMutation({
 *   variables: {
 *   },
 * });
 */
export function useRequestRoboticonScoresMutation(baseOptions?: Apollo.MutationHookOptions<RequestRoboticonScoresMutation, RequestRoboticonScoresMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<RequestRoboticonScoresMutation, RequestRoboticonScoresMutationVariables>(RequestRoboticonScoresDocument, options);
      }
export type RequestRoboticonScoresMutationHookResult = ReturnType<typeof useRequestRoboticonScoresMutation>;
export type RequestRoboticonScoresMutationResult = Apollo.MutationResult<RequestRoboticonScoresMutation>;
export type RequestRoboticonScoresMutationOptions = Apollo.BaseMutationOptions<RequestRoboticonScoresMutation, RequestRoboticonScoresMutationVariables>;
export const RoboticonGameStateDocument = gql`
    subscription RoboticonGameState {
  subscribe(topic: "roboticonGameState")
}
    `;

/**
 * __useRoboticonGameStateSubscription__
 *
 * To run a query within a React component, call `useRoboticonGameStateSubscription` and pass it any options that fit your needs.
 * When your component renders, `useRoboticonGameStateSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useRoboticonGameStateSubscription({
 *   variables: {
 *   },
 * });
 */
export function useRoboticonGameStateSubscription(baseOptions?: Apollo.SubscriptionHookOptions<RoboticonGameStateSubscription, RoboticonGameStateSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<RoboticonGameStateSubscription, RoboticonGameStateSubscriptionVariables>(RoboticonGameStateDocument, options);
      }
export type RoboticonGameStateSubscriptionHookResult = ReturnType<typeof useRoboticonGameStateSubscription>;
export type RoboticonGameStateSubscriptionResult = Apollo.SubscriptionResult<RoboticonGameStateSubscription>;
export const RoboticonScoresDocument = gql`
    subscription RoboticonScores {
  subscribe(topic: "roboticonReplyScores")
}
    `;

/**
 * __useRoboticonScoresSubscription__
 *
 * To run a query within a React component, call `useRoboticonScoresSubscription` and pass it any options that fit your needs.
 * When your component renders, `useRoboticonScoresSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useRoboticonScoresSubscription({
 *   variables: {
 *   },
 * });
 */
export function useRoboticonScoresSubscription(baseOptions?: Apollo.SubscriptionHookOptions<RoboticonScoresSubscription, RoboticonScoresSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<RoboticonScoresSubscription, RoboticonScoresSubscriptionVariables>(RoboticonScoresDocument, options);
      }
export type RoboticonScoresSubscriptionHookResult = ReturnType<typeof useRoboticonScoresSubscription>;
export type RoboticonScoresSubscriptionResult = Apollo.SubscriptionResult<RoboticonScoresSubscription>;
export const SignInDocument = gql`
    mutation SignIn($username: String!, $password: String!) {
  signIn(username: $username, password: $password)
}
    `;
export type SignInMutationFn = Apollo.MutationFunction<SignInMutation, SignInMutationVariables>;

/**
 * __useSignInMutation__
 *
 * To run a mutation, you first call `useSignInMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useSignInMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [signInMutation, { data, loading, error }] = useSignInMutation({
 *   variables: {
 *      username: // value for 'username'
 *      password: // value for 'password'
 *   },
 * });
 */
export function useSignInMutation(baseOptions?: Apollo.MutationHookOptions<SignInMutation, SignInMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<SignInMutation, SignInMutationVariables>(SignInDocument, options);
      }
export type SignInMutationHookResult = ReturnType<typeof useSignInMutation>;
export type SignInMutationResult = Apollo.MutationResult<SignInMutation>;
export type SignInMutationOptions = Apollo.BaseMutationOptions<SignInMutation, SignInMutationVariables>;
export const StartRoboticonGameDocument = gql`
    mutation StartRoboticonGame {
  publish(topic: "roboticonStartGame", message: "150")
}
    `;
export type StartRoboticonGameMutationFn = Apollo.MutationFunction<StartRoboticonGameMutation, StartRoboticonGameMutationVariables>;

/**
 * __useStartRoboticonGameMutation__
 *
 * To run a mutation, you first call `useStartRoboticonGameMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useStartRoboticonGameMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [startRoboticonGameMutation, { data, loading, error }] = useStartRoboticonGameMutation({
 *   variables: {
 *   },
 * });
 */
export function useStartRoboticonGameMutation(baseOptions?: Apollo.MutationHookOptions<StartRoboticonGameMutation, StartRoboticonGameMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<StartRoboticonGameMutation, StartRoboticonGameMutationVariables>(StartRoboticonGameDocument, options);
      }
export type StartRoboticonGameMutationHookResult = ReturnType<typeof useStartRoboticonGameMutation>;
export type StartRoboticonGameMutationResult = Apollo.MutationResult<StartRoboticonGameMutation>;
export type StartRoboticonGameMutationOptions = Apollo.BaseMutationOptions<StartRoboticonGameMutation, StartRoboticonGameMutationVariables>;
export const StopRoboticonGameDocument = gql`
    mutation StopRoboticonGame {
  publish(topic: "roboticonStopGame", message: "true")
}
    `;
export type StopRoboticonGameMutationFn = Apollo.MutationFunction<StopRoboticonGameMutation, StopRoboticonGameMutationVariables>;

/**
 * __useStopRoboticonGameMutation__
 *
 * To run a mutation, you first call `useStopRoboticonGameMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useStopRoboticonGameMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [stopRoboticonGameMutation, { data, loading, error }] = useStopRoboticonGameMutation({
 *   variables: {
 *   },
 * });
 */
export function useStopRoboticonGameMutation(baseOptions?: Apollo.MutationHookOptions<StopRoboticonGameMutation, StopRoboticonGameMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<StopRoboticonGameMutation, StopRoboticonGameMutationVariables>(StopRoboticonGameDocument, options);
      }
export type StopRoboticonGameMutationHookResult = ReturnType<typeof useStopRoboticonGameMutation>;
export type StopRoboticonGameMutationResult = Apollo.MutationResult<StopRoboticonGameMutation>;
export type StopRoboticonGameMutationOptions = Apollo.BaseMutationOptions<StopRoboticonGameMutation, StopRoboticonGameMutationVariables>;
export const SwitchRoboticonGameDocument = gql`
    mutation SwitchRoboticonGame($game: String!) {
  publish(topic: "roboticonSwitchGames", message: $game)
}
    `;
export type SwitchRoboticonGameMutationFn = Apollo.MutationFunction<SwitchRoboticonGameMutation, SwitchRoboticonGameMutationVariables>;

/**
 * __useSwitchRoboticonGameMutation__
 *
 * To run a mutation, you first call `useSwitchRoboticonGameMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useSwitchRoboticonGameMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [switchRoboticonGameMutation, { data, loading, error }] = useSwitchRoboticonGameMutation({
 *   variables: {
 *      game: // value for 'game'
 *   },
 * });
 */
export function useSwitchRoboticonGameMutation(baseOptions?: Apollo.MutationHookOptions<SwitchRoboticonGameMutation, SwitchRoboticonGameMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<SwitchRoboticonGameMutation, SwitchRoboticonGameMutationVariables>(SwitchRoboticonGameDocument, options);
      }
export type SwitchRoboticonGameMutationHookResult = ReturnType<typeof useSwitchRoboticonGameMutation>;
export type SwitchRoboticonGameMutationResult = Apollo.MutationResult<SwitchRoboticonGameMutation>;
export type SwitchRoboticonGameMutationOptions = Apollo.BaseMutationOptions<SwitchRoboticonGameMutation, SwitchRoboticonGameMutationVariables>;
export const UpdateRoboticonScoreDocument = gql`
    mutation UpdateRoboticonScore($data: String!) {
  publish(topic: "roboticonUpdateScore", message: $data)
}
    `;
export type UpdateRoboticonScoreMutationFn = Apollo.MutationFunction<UpdateRoboticonScoreMutation, UpdateRoboticonScoreMutationVariables>;

/**
 * __useUpdateRoboticonScoreMutation__
 *
 * To run a mutation, you first call `useUpdateRoboticonScoreMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdateRoboticonScoreMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updateRoboticonScoreMutation, { data, loading, error }] = useUpdateRoboticonScoreMutation({
 *   variables: {
 *      data: // value for 'data'
 *   },
 * });
 */
export function useUpdateRoboticonScoreMutation(baseOptions?: Apollo.MutationHookOptions<UpdateRoboticonScoreMutation, UpdateRoboticonScoreMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<UpdateRoboticonScoreMutation, UpdateRoboticonScoreMutationVariables>(UpdateRoboticonScoreDocument, options);
      }
export type UpdateRoboticonScoreMutationHookResult = ReturnType<typeof useUpdateRoboticonScoreMutation>;
export type UpdateRoboticonScoreMutationResult = Apollo.MutationResult<UpdateRoboticonScoreMutation>;
export type UpdateRoboticonScoreMutationOptions = Apollo.BaseMutationOptions<UpdateRoboticonScoreMutation, UpdateRoboticonScoreMutationVariables>;