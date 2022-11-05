/* eslint-disable */
import {
  CallOptions,
  ChannelCredentials,
  ChannelOptions,
  Client,
  ClientReadableStream,
  ClientUnaryCall,
  handleServerStreamingCall,
  handleUnaryCall,
  makeGenericClientConstructor,
  Metadata,
  ServiceError,
  UntypedServiceImplementation,
} from "@grpc/grpc-js";
import Long from "long";
import _m0 from "protobufjs/minimal";

export enum Mode {
  TELEOP = 0,
  TEST = 1,
  AUTONOMOUS = 2,
  UNRECOGNIZED = -1,
}

export function modeFromJSON(object: any): Mode {
  switch (object) {
    case 0:
    case "TELEOP":
      return Mode.TELEOP;
    case 1:
    case "TEST":
      return Mode.TEST;
    case 2:
    case "AUTONOMOUS":
      return Mode.AUTONOMOUS;
    case -1:
    case "UNRECOGNIZED":
    default:
      return Mode.UNRECOGNIZED;
  }
}

export function modeToJSON(object: Mode): string {
  switch (object) {
    case Mode.TELEOP:
      return "TELEOP";
    case Mode.TEST:
      return "TEST";
    case Mode.AUTONOMOUS:
      return "AUTONOMOUS";
    case Mode.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum DriverstationStatus {
  GOOD = 0,
  BAD = 1,
  WAITING = 2,
  UNRECOGNIZED = -1,
}

export function driverstationStatusFromJSON(object: any): DriverstationStatus {
  switch (object) {
    case 0:
    case "GOOD":
      return DriverstationStatus.GOOD;
    case 1:
    case "BAD":
      return DriverstationStatus.BAD;
    case 2:
    case "WAITING":
      return DriverstationStatus.WAITING;
    case -1:
    case "UNRECOGNIZED":
    default:
      return DriverstationStatus.UNRECOGNIZED;
  }
}

export function driverstationStatusToJSON(object: DriverstationStatus): string {
  switch (object) {
    case DriverstationStatus.GOOD:
      return "GOOD";
    case DriverstationStatus.BAD:
      return "BAD";
    case DriverstationStatus.WAITING:
      return "WAITING";
    case DriverstationStatus.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum AllianceStation {
  RED1 = 0,
  RED2 = 1,
  RED3 = 2,
  BLUE1 = 3,
  BLUE2 = 4,
  BLUE3 = 5,
  NONE = 6,
  UNRECOGNIZED = -1,
}

export function allianceStationFromJSON(object: any): AllianceStation {
  switch (object) {
    case 0:
    case "RED1":
      return AllianceStation.RED1;
    case 1:
    case "RED2":
      return AllianceStation.RED2;
    case 2:
    case "RED3":
      return AllianceStation.RED3;
    case 3:
    case "BLUE1":
      return AllianceStation.BLUE1;
    case 4:
    case "BLUE2":
      return AllianceStation.BLUE2;
    case 5:
    case "BLUE3":
      return AllianceStation.BLUE3;
    case 6:
    case "NONE":
      return AllianceStation.NONE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return AllianceStation.UNRECOGNIZED;
  }
}

export function allianceStationToJSON(object: AllianceStation): string {
  switch (object) {
    case AllianceStation.RED1:
      return "RED1";
    case AllianceStation.RED2:
      return "RED2";
    case AllianceStation.RED3:
      return "RED3";
    case AllianceStation.BLUE1:
      return "BLUE1";
    case AllianceStation.BLUE2:
      return "BLUE2";
    case AllianceStation.BLUE3:
      return "BLUE3";
    case AllianceStation.NONE:
      return "NONE";
    case AllianceStation.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum TournamentLevel {
  TESTING = 0,
  PRACTICE = 1,
  QUALIFICATION = 2,
  PLAYOFF = 3,
  UNRECOGNIZED = -1,
}

export function tournamentLevelFromJSON(object: any): TournamentLevel {
  switch (object) {
    case 0:
    case "TESTING":
      return TournamentLevel.TESTING;
    case 1:
    case "PRACTICE":
      return TournamentLevel.PRACTICE;
    case 2:
    case "QUALIFICATION":
      return TournamentLevel.QUALIFICATION;
    case 3:
    case "PLAYOFF":
      return TournamentLevel.PLAYOFF;
    case -1:
    case "UNRECOGNIZED":
    default:
      return TournamentLevel.UNRECOGNIZED;
  }
}

export function tournamentLevelToJSON(object: TournamentLevel): string {
  switch (object) {
    case TournamentLevel.TESTING:
      return "TESTING";
    case TournamentLevel.PRACTICE:
      return "PRACTICE";
    case TournamentLevel.QUALIFICATION:
      return "QUALIFICATION";
    case TournamentLevel.PLAYOFF:
      return "PLAYOFF";
    case TournamentLevel.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum DriverStationQueryType {
  TEAMNUMBER = 0,
  ALLIANCESTATION = 1,
  UNRECOGNIZED = -1,
}

export function driverStationQueryTypeFromJSON(object: any): DriverStationQueryType {
  switch (object) {
    case 0:
    case "TEAMNUMBER":
      return DriverStationQueryType.TEAMNUMBER;
    case 1:
    case "ALLIANCESTATION":
      return DriverStationQueryType.ALLIANCESTATION;
    case -1:
    case "UNRECOGNIZED":
    default:
      return DriverStationQueryType.UNRECOGNIZED;
  }
}

export function driverStationQueryTypeToJSON(object: DriverStationQueryType): string {
  switch (object) {
    case DriverStationQueryType.TEAMNUMBER:
      return "TEAMNUMBER";
    case DriverStationQueryType.ALLIANCESTATION:
      return "ALLIANCESTATION";
    case DriverStationQueryType.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface Empty {
}

/** Plugin Registration Messages */
export interface PluginMetadata {
  id: string;
  name?: string | undefined;
  description?: string | undefined;
  readme?: string | undefined;
  version?: string | undefined;
  authors: string[];
  srcUrl?: string | undefined;
  docsUrl?: string | undefined;
}

export interface PluginRegistrationRequest {
  registrationToken: string;
  plugin?: PluginMetadata;
}

export interface PluginRegistrationResponse {
  token: string;
}

export interface JSONRpcMessage {
  pluginId: string;
  data: string;
}

export interface FieldState {
  eventName: string;
  tournamentLevel: TournamentLevel;
  matchNumber: number;
  playNumber: number;
  timer?: DiffTimer;
}

export interface FieldConfiguration {
  eventName: string;
  tournamentLevel: TournamentLevel;
  matchNumber: number;
  playNumber: number;
}

export interface FieldTimerUpdate {
  timeRemaining?: number | undefined;
  running: boolean;
}

export interface DriverStations {
  driverStations: DriverStation[];
}

export interface DriverStationQuery {
  queryType: DriverStationQueryType;
  teamNumber: number;
  allianceStation: AllianceStation;
}

export interface DriverStationParams {
  teamNumber: number;
  allianceStation: AllianceStation;
}

export interface DriverStationUpdateExpectedIP {
  teamNumber: number;
  expectedIp: string;
}

export interface DriverStationUpdateMode {
  teamNumber: number;
  mode: Mode;
}

export interface DriverStation {
  teamNumber: number;
  allianceStation: AllianceStation;
  expectedIp?: string | undefined;
  connection?: DriverStationConnection | undefined;
  confirmedState?: DriverStationConfirmedState | undefined;
}

export interface DriverStationConnection {
  alive: boolean;
  ip: string;
  outgoingSequenceNum: number;
}

export interface DriverStationConfirmedState {
  isEmergencyStopped: boolean;
  robotCommunicationsActive: boolean;
  canPingRadio: boolean;
  canPingRio: boolean;
  isEnabled: boolean;
  mode: Mode;
  teamNumber: number;
  batteryVoltage: number;
}

export interface EnablerQuery {
  id: string;
}

export interface EnablerConfig {
  id: string;
  name: string;
  allEnabler?: AllEnablerConfig | undefined;
  teamNumberEnabler?: TeamNumberEnablerConfig | undefined;
  allianceStationEnabler?: AllianceStationEnablerConfig | undefined;
}

export interface AllEnablerConfig {
  active: boolean;
}

export interface TeamNumberEnablerConfig {
  approvedTeamNumbers: number[];
}

export interface AllianceStationEnablerConfig {
  approvedStations: AllianceStation[];
}

export interface EstopperQuery {
  id: string;
}

export interface EstopperConfig {
  id: string;
  name: string;
  allEstopper?: AllEstopperConfig | undefined;
  teamNumberEstopper?: TeamNumberEstopperConfig | undefined;
  allianceStationEstopper?: AllianceStationEstopperConfig | undefined;
}

export interface AllEstopperConfig {
  active: boolean;
}

export interface TeamNumberEstopperConfig {
  estoppedTeamNumbers: number[];
}

export interface AllianceStationEstopperConfig {
  estoppedStations: AllianceStation[];
}

/**
 * / DiffTimer is a way to represent the game time remaining in a way that can easily be synced
 * / between different displaying devices provided they all use NTP.
 * /
 * / If started_at is 0, then the timer is currently frozen and time_remaining represents the milliseconds that should be displayed
 * /
 * / If started_at is >0, then the timer is currently running and time_remaining represents the milliseconds
 * / that the clock had at the time specified by started_at (epoch time in ms).
 */
export interface DiffTimer {
  startedAt: number;
  timeRemaining: number;
}

function createBaseEmpty(): Empty {
  return {};
}

export const Empty = {
  encode(_: Empty, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Empty {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEmpty();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): Empty {
    return {};
  },

  toJSON(_: Empty): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Empty>, I>>(_: I): Empty {
    const message = createBaseEmpty();
    return message;
  },
};

function createBasePluginMetadata(): PluginMetadata {
  return {
    id: "",
    name: undefined,
    description: undefined,
    readme: undefined,
    version: undefined,
    authors: [],
    srcUrl: undefined,
    docsUrl: undefined,
  };
}

export const PluginMetadata = {
  encode(message: PluginMetadata, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    if (message.name !== undefined) {
      writer.uint32(18).string(message.name);
    }
    if (message.description !== undefined) {
      writer.uint32(26).string(message.description);
    }
    if (message.readme !== undefined) {
      writer.uint32(34).string(message.readme);
    }
    if (message.version !== undefined) {
      writer.uint32(42).string(message.version);
    }
    for (const v of message.authors) {
      writer.uint32(50).string(v!);
    }
    if (message.srcUrl !== undefined) {
      writer.uint32(58).string(message.srcUrl);
    }
    if (message.docsUrl !== undefined) {
      writer.uint32(66).string(message.docsUrl);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PluginMetadata {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePluginMetadata();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.description = reader.string();
          break;
        case 4:
          message.readme = reader.string();
          break;
        case 5:
          message.version = reader.string();
          break;
        case 6:
          message.authors.push(reader.string());
          break;
        case 7:
          message.srcUrl = reader.string();
          break;
        case 8:
          message.docsUrl = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PluginMetadata {
    return {
      id: isSet(object.id) ? String(object.id) : "",
      name: isSet(object.name) ? String(object.name) : undefined,
      description: isSet(object.description) ? String(object.description) : undefined,
      readme: isSet(object.readme) ? String(object.readme) : undefined,
      version: isSet(object.version) ? String(object.version) : undefined,
      authors: Array.isArray(object?.authors) ? object.authors.map((e: any) => String(e)) : [],
      srcUrl: isSet(object.srcUrl) ? String(object.srcUrl) : undefined,
      docsUrl: isSet(object.docsUrl) ? String(object.docsUrl) : undefined,
    };
  },

  toJSON(message: PluginMetadata): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    message.description !== undefined && (obj.description = message.description);
    message.readme !== undefined && (obj.readme = message.readme);
    message.version !== undefined && (obj.version = message.version);
    if (message.authors) {
      obj.authors = message.authors.map((e) => e);
    } else {
      obj.authors = [];
    }
    message.srcUrl !== undefined && (obj.srcUrl = message.srcUrl);
    message.docsUrl !== undefined && (obj.docsUrl = message.docsUrl);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PluginMetadata>, I>>(object: I): PluginMetadata {
    const message = createBasePluginMetadata();
    message.id = object.id ?? "";
    message.name = object.name ?? undefined;
    message.description = object.description ?? undefined;
    message.readme = object.readme ?? undefined;
    message.version = object.version ?? undefined;
    message.authors = object.authors?.map((e) => e) || [];
    message.srcUrl = object.srcUrl ?? undefined;
    message.docsUrl = object.docsUrl ?? undefined;
    return message;
  },
};

function createBasePluginRegistrationRequest(): PluginRegistrationRequest {
  return { registrationToken: "", plugin: undefined };
}

export const PluginRegistrationRequest = {
  encode(message: PluginRegistrationRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.registrationToken !== "") {
      writer.uint32(10).string(message.registrationToken);
    }
    if (message.plugin !== undefined) {
      PluginMetadata.encode(message.plugin, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PluginRegistrationRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePluginRegistrationRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.registrationToken = reader.string();
          break;
        case 2:
          message.plugin = PluginMetadata.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PluginRegistrationRequest {
    return {
      registrationToken: isSet(object.registrationToken) ? String(object.registrationToken) : "",
      plugin: isSet(object.plugin) ? PluginMetadata.fromJSON(object.plugin) : undefined,
    };
  },

  toJSON(message: PluginRegistrationRequest): unknown {
    const obj: any = {};
    message.registrationToken !== undefined && (obj.registrationToken = message.registrationToken);
    message.plugin !== undefined && (obj.plugin = message.plugin ? PluginMetadata.toJSON(message.plugin) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PluginRegistrationRequest>, I>>(object: I): PluginRegistrationRequest {
    const message = createBasePluginRegistrationRequest();
    message.registrationToken = object.registrationToken ?? "";
    message.plugin = (object.plugin !== undefined && object.plugin !== null)
      ? PluginMetadata.fromPartial(object.plugin)
      : undefined;
    return message;
  },
};

function createBasePluginRegistrationResponse(): PluginRegistrationResponse {
  return { token: "" };
}

export const PluginRegistrationResponse = {
  encode(message: PluginRegistrationResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.token !== "") {
      writer.uint32(10).string(message.token);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PluginRegistrationResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePluginRegistrationResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.token = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PluginRegistrationResponse {
    return { token: isSet(object.token) ? String(object.token) : "" };
  },

  toJSON(message: PluginRegistrationResponse): unknown {
    const obj: any = {};
    message.token !== undefined && (obj.token = message.token);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<PluginRegistrationResponse>, I>>(object: I): PluginRegistrationResponse {
    const message = createBasePluginRegistrationResponse();
    message.token = object.token ?? "";
    return message;
  },
};

function createBaseJSONRpcMessage(): JSONRpcMessage {
  return { pluginId: "", data: "" };
}

export const JSONRpcMessage = {
  encode(message: JSONRpcMessage, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.pluginId !== "") {
      writer.uint32(10).string(message.pluginId);
    }
    if (message.data !== "") {
      writer.uint32(18).string(message.data);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): JSONRpcMessage {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseJSONRpcMessage();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.pluginId = reader.string();
          break;
        case 2:
          message.data = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): JSONRpcMessage {
    return {
      pluginId: isSet(object.pluginId) ? String(object.pluginId) : "",
      data: isSet(object.data) ? String(object.data) : "",
    };
  },

  toJSON(message: JSONRpcMessage): unknown {
    const obj: any = {};
    message.pluginId !== undefined && (obj.pluginId = message.pluginId);
    message.data !== undefined && (obj.data = message.data);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<JSONRpcMessage>, I>>(object: I): JSONRpcMessage {
    const message = createBaseJSONRpcMessage();
    message.pluginId = object.pluginId ?? "";
    message.data = object.data ?? "";
    return message;
  },
};

function createBaseFieldState(): FieldState {
  return { eventName: "", tournamentLevel: 0, matchNumber: 0, playNumber: 0, timer: undefined };
}

export const FieldState = {
  encode(message: FieldState, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.eventName !== "") {
      writer.uint32(10).string(message.eventName);
    }
    if (message.tournamentLevel !== 0) {
      writer.uint32(16).int32(message.tournamentLevel);
    }
    if (message.matchNumber !== 0) {
      writer.uint32(24).uint32(message.matchNumber);
    }
    if (message.playNumber !== 0) {
      writer.uint32(32).uint32(message.playNumber);
    }
    if (message.timer !== undefined) {
      DiffTimer.encode(message.timer, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FieldState {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFieldState();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.eventName = reader.string();
          break;
        case 2:
          message.tournamentLevel = reader.int32() as any;
          break;
        case 3:
          message.matchNumber = reader.uint32();
          break;
        case 4:
          message.playNumber = reader.uint32();
          break;
        case 5:
          message.timer = DiffTimer.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FieldState {
    return {
      eventName: isSet(object.eventName) ? String(object.eventName) : "",
      tournamentLevel: isSet(object.tournamentLevel) ? tournamentLevelFromJSON(object.tournamentLevel) : 0,
      matchNumber: isSet(object.matchNumber) ? Number(object.matchNumber) : 0,
      playNumber: isSet(object.playNumber) ? Number(object.playNumber) : 0,
      timer: isSet(object.timer) ? DiffTimer.fromJSON(object.timer) : undefined,
    };
  },

  toJSON(message: FieldState): unknown {
    const obj: any = {};
    message.eventName !== undefined && (obj.eventName = message.eventName);
    message.tournamentLevel !== undefined && (obj.tournamentLevel = tournamentLevelToJSON(message.tournamentLevel));
    message.matchNumber !== undefined && (obj.matchNumber = Math.round(message.matchNumber));
    message.playNumber !== undefined && (obj.playNumber = Math.round(message.playNumber));
    message.timer !== undefined && (obj.timer = message.timer ? DiffTimer.toJSON(message.timer) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FieldState>, I>>(object: I): FieldState {
    const message = createBaseFieldState();
    message.eventName = object.eventName ?? "";
    message.tournamentLevel = object.tournamentLevel ?? 0;
    message.matchNumber = object.matchNumber ?? 0;
    message.playNumber = object.playNumber ?? 0;
    message.timer = (object.timer !== undefined && object.timer !== null)
      ? DiffTimer.fromPartial(object.timer)
      : undefined;
    return message;
  },
};

function createBaseFieldConfiguration(): FieldConfiguration {
  return { eventName: "", tournamentLevel: 0, matchNumber: 0, playNumber: 0 };
}

export const FieldConfiguration = {
  encode(message: FieldConfiguration, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.eventName !== "") {
      writer.uint32(10).string(message.eventName);
    }
    if (message.tournamentLevel !== 0) {
      writer.uint32(16).int32(message.tournamentLevel);
    }
    if (message.matchNumber !== 0) {
      writer.uint32(24).uint32(message.matchNumber);
    }
    if (message.playNumber !== 0) {
      writer.uint32(32).uint32(message.playNumber);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FieldConfiguration {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFieldConfiguration();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.eventName = reader.string();
          break;
        case 2:
          message.tournamentLevel = reader.int32() as any;
          break;
        case 3:
          message.matchNumber = reader.uint32();
          break;
        case 4:
          message.playNumber = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FieldConfiguration {
    return {
      eventName: isSet(object.eventName) ? String(object.eventName) : "",
      tournamentLevel: isSet(object.tournamentLevel) ? tournamentLevelFromJSON(object.tournamentLevel) : 0,
      matchNumber: isSet(object.matchNumber) ? Number(object.matchNumber) : 0,
      playNumber: isSet(object.playNumber) ? Number(object.playNumber) : 0,
    };
  },

  toJSON(message: FieldConfiguration): unknown {
    const obj: any = {};
    message.eventName !== undefined && (obj.eventName = message.eventName);
    message.tournamentLevel !== undefined && (obj.tournamentLevel = tournamentLevelToJSON(message.tournamentLevel));
    message.matchNumber !== undefined && (obj.matchNumber = Math.round(message.matchNumber));
    message.playNumber !== undefined && (obj.playNumber = Math.round(message.playNumber));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FieldConfiguration>, I>>(object: I): FieldConfiguration {
    const message = createBaseFieldConfiguration();
    message.eventName = object.eventName ?? "";
    message.tournamentLevel = object.tournamentLevel ?? 0;
    message.matchNumber = object.matchNumber ?? 0;
    message.playNumber = object.playNumber ?? 0;
    return message;
  },
};

function createBaseFieldTimerUpdate(): FieldTimerUpdate {
  return { timeRemaining: undefined, running: false };
}

export const FieldTimerUpdate = {
  encode(message: FieldTimerUpdate, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.timeRemaining !== undefined) {
      writer.uint32(8).uint64(message.timeRemaining);
    }
    if (message.running === true) {
      writer.uint32(16).bool(message.running);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FieldTimerUpdate {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFieldTimerUpdate();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.timeRemaining = longToNumber(reader.uint64() as Long);
          break;
        case 2:
          message.running = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FieldTimerUpdate {
    return {
      timeRemaining: isSet(object.timeRemaining) ? Number(object.timeRemaining) : undefined,
      running: isSet(object.running) ? Boolean(object.running) : false,
    };
  },

  toJSON(message: FieldTimerUpdate): unknown {
    const obj: any = {};
    message.timeRemaining !== undefined && (obj.timeRemaining = Math.round(message.timeRemaining));
    message.running !== undefined && (obj.running = message.running);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FieldTimerUpdate>, I>>(object: I): FieldTimerUpdate {
    const message = createBaseFieldTimerUpdate();
    message.timeRemaining = object.timeRemaining ?? undefined;
    message.running = object.running ?? false;
    return message;
  },
};

function createBaseDriverStations(): DriverStations {
  return { driverStations: [] };
}

export const DriverStations = {
  encode(message: DriverStations, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.driverStations) {
      DriverStation.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStations {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStations();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.driverStations.push(DriverStation.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStations {
    return {
      driverStations: Array.isArray(object?.driverStations)
        ? object.driverStations.map((e: any) => DriverStation.fromJSON(e))
        : [],
    };
  },

  toJSON(message: DriverStations): unknown {
    const obj: any = {};
    if (message.driverStations) {
      obj.driverStations = message.driverStations.map((e) => e ? DriverStation.toJSON(e) : undefined);
    } else {
      obj.driverStations = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStations>, I>>(object: I): DriverStations {
    const message = createBaseDriverStations();
    message.driverStations = object.driverStations?.map((e) => DriverStation.fromPartial(e)) || [];
    return message;
  },
};

function createBaseDriverStationQuery(): DriverStationQuery {
  return { queryType: 0, teamNumber: 0, allianceStation: 0 };
}

export const DriverStationQuery = {
  encode(message: DriverStationQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.queryType !== 0) {
      writer.uint32(8).int32(message.queryType);
    }
    if (message.teamNumber !== 0) {
      writer.uint32(16).uint32(message.teamNumber);
    }
    if (message.allianceStation !== 0) {
      writer.uint32(24).int32(message.allianceStation);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStationQuery {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStationQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.queryType = reader.int32() as any;
          break;
        case 2:
          message.teamNumber = reader.uint32();
          break;
        case 3:
          message.allianceStation = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStationQuery {
    return {
      queryType: isSet(object.queryType) ? driverStationQueryTypeFromJSON(object.queryType) : 0,
      teamNumber: isSet(object.teamNumber) ? Number(object.teamNumber) : 0,
      allianceStation: isSet(object.allianceStation) ? allianceStationFromJSON(object.allianceStation) : 0,
    };
  },

  toJSON(message: DriverStationQuery): unknown {
    const obj: any = {};
    message.queryType !== undefined && (obj.queryType = driverStationQueryTypeToJSON(message.queryType));
    message.teamNumber !== undefined && (obj.teamNumber = Math.round(message.teamNumber));
    message.allianceStation !== undefined && (obj.allianceStation = allianceStationToJSON(message.allianceStation));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStationQuery>, I>>(object: I): DriverStationQuery {
    const message = createBaseDriverStationQuery();
    message.queryType = object.queryType ?? 0;
    message.teamNumber = object.teamNumber ?? 0;
    message.allianceStation = object.allianceStation ?? 0;
    return message;
  },
};

function createBaseDriverStationParams(): DriverStationParams {
  return { teamNumber: 0, allianceStation: 0 };
}

export const DriverStationParams = {
  encode(message: DriverStationParams, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.teamNumber !== 0) {
      writer.uint32(8).uint32(message.teamNumber);
    }
    if (message.allianceStation !== 0) {
      writer.uint32(16).int32(message.allianceStation);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStationParams {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStationParams();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.teamNumber = reader.uint32();
          break;
        case 2:
          message.allianceStation = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStationParams {
    return {
      teamNumber: isSet(object.teamNumber) ? Number(object.teamNumber) : 0,
      allianceStation: isSet(object.allianceStation) ? allianceStationFromJSON(object.allianceStation) : 0,
    };
  },

  toJSON(message: DriverStationParams): unknown {
    const obj: any = {};
    message.teamNumber !== undefined && (obj.teamNumber = Math.round(message.teamNumber));
    message.allianceStation !== undefined && (obj.allianceStation = allianceStationToJSON(message.allianceStation));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStationParams>, I>>(object: I): DriverStationParams {
    const message = createBaseDriverStationParams();
    message.teamNumber = object.teamNumber ?? 0;
    message.allianceStation = object.allianceStation ?? 0;
    return message;
  },
};

function createBaseDriverStationUpdateExpectedIP(): DriverStationUpdateExpectedIP {
  return { teamNumber: 0, expectedIp: "" };
}

export const DriverStationUpdateExpectedIP = {
  encode(message: DriverStationUpdateExpectedIP, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.teamNumber !== 0) {
      writer.uint32(8).uint32(message.teamNumber);
    }
    if (message.expectedIp !== "") {
      writer.uint32(18).string(message.expectedIp);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStationUpdateExpectedIP {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStationUpdateExpectedIP();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.teamNumber = reader.uint32();
          break;
        case 2:
          message.expectedIp = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStationUpdateExpectedIP {
    return {
      teamNumber: isSet(object.teamNumber) ? Number(object.teamNumber) : 0,
      expectedIp: isSet(object.expectedIp) ? String(object.expectedIp) : "",
    };
  },

  toJSON(message: DriverStationUpdateExpectedIP): unknown {
    const obj: any = {};
    message.teamNumber !== undefined && (obj.teamNumber = Math.round(message.teamNumber));
    message.expectedIp !== undefined && (obj.expectedIp = message.expectedIp);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStationUpdateExpectedIP>, I>>(
    object: I,
  ): DriverStationUpdateExpectedIP {
    const message = createBaseDriverStationUpdateExpectedIP();
    message.teamNumber = object.teamNumber ?? 0;
    message.expectedIp = object.expectedIp ?? "";
    return message;
  },
};

function createBaseDriverStationUpdateMode(): DriverStationUpdateMode {
  return { teamNumber: 0, mode: 0 };
}

export const DriverStationUpdateMode = {
  encode(message: DriverStationUpdateMode, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.teamNumber !== 0) {
      writer.uint32(8).uint32(message.teamNumber);
    }
    if (message.mode !== 0) {
      writer.uint32(16).int32(message.mode);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStationUpdateMode {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStationUpdateMode();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.teamNumber = reader.uint32();
          break;
        case 2:
          message.mode = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStationUpdateMode {
    return {
      teamNumber: isSet(object.teamNumber) ? Number(object.teamNumber) : 0,
      mode: isSet(object.mode) ? modeFromJSON(object.mode) : 0,
    };
  },

  toJSON(message: DriverStationUpdateMode): unknown {
    const obj: any = {};
    message.teamNumber !== undefined && (obj.teamNumber = Math.round(message.teamNumber));
    message.mode !== undefined && (obj.mode = modeToJSON(message.mode));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStationUpdateMode>, I>>(object: I): DriverStationUpdateMode {
    const message = createBaseDriverStationUpdateMode();
    message.teamNumber = object.teamNumber ?? 0;
    message.mode = object.mode ?? 0;
    return message;
  },
};

function createBaseDriverStation(): DriverStation {
  return { teamNumber: 0, allianceStation: 0, expectedIp: undefined, connection: undefined, confirmedState: undefined };
}

export const DriverStation = {
  encode(message: DriverStation, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.teamNumber !== 0) {
      writer.uint32(8).uint32(message.teamNumber);
    }
    if (message.allianceStation !== 0) {
      writer.uint32(16).int32(message.allianceStation);
    }
    if (message.expectedIp !== undefined) {
      writer.uint32(26).string(message.expectedIp);
    }
    if (message.connection !== undefined) {
      DriverStationConnection.encode(message.connection, writer.uint32(34).fork()).ldelim();
    }
    if (message.confirmedState !== undefined) {
      DriverStationConfirmedState.encode(message.confirmedState, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStation {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStation();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.teamNumber = reader.uint32();
          break;
        case 2:
          message.allianceStation = reader.int32() as any;
          break;
        case 3:
          message.expectedIp = reader.string();
          break;
        case 4:
          message.connection = DriverStationConnection.decode(reader, reader.uint32());
          break;
        case 5:
          message.confirmedState = DriverStationConfirmedState.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStation {
    return {
      teamNumber: isSet(object.teamNumber) ? Number(object.teamNumber) : 0,
      allianceStation: isSet(object.allianceStation) ? allianceStationFromJSON(object.allianceStation) : 0,
      expectedIp: isSet(object.expectedIp) ? String(object.expectedIp) : undefined,
      connection: isSet(object.connection) ? DriverStationConnection.fromJSON(object.connection) : undefined,
      confirmedState: isSet(object.confirmedState)
        ? DriverStationConfirmedState.fromJSON(object.confirmedState)
        : undefined,
    };
  },

  toJSON(message: DriverStation): unknown {
    const obj: any = {};
    message.teamNumber !== undefined && (obj.teamNumber = Math.round(message.teamNumber));
    message.allianceStation !== undefined && (obj.allianceStation = allianceStationToJSON(message.allianceStation));
    message.expectedIp !== undefined && (obj.expectedIp = message.expectedIp);
    message.connection !== undefined &&
      (obj.connection = message.connection ? DriverStationConnection.toJSON(message.connection) : undefined);
    message.confirmedState !== undefined && (obj.confirmedState = message.confirmedState
      ? DriverStationConfirmedState.toJSON(message.confirmedState)
      : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStation>, I>>(object: I): DriverStation {
    const message = createBaseDriverStation();
    message.teamNumber = object.teamNumber ?? 0;
    message.allianceStation = object.allianceStation ?? 0;
    message.expectedIp = object.expectedIp ?? undefined;
    message.connection = (object.connection !== undefined && object.connection !== null)
      ? DriverStationConnection.fromPartial(object.connection)
      : undefined;
    message.confirmedState = (object.confirmedState !== undefined && object.confirmedState !== null)
      ? DriverStationConfirmedState.fromPartial(object.confirmedState)
      : undefined;
    return message;
  },
};

function createBaseDriverStationConnection(): DriverStationConnection {
  return { alive: false, ip: "", outgoingSequenceNum: 0 };
}

export const DriverStationConnection = {
  encode(message: DriverStationConnection, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.alive === true) {
      writer.uint32(8).bool(message.alive);
    }
    if (message.ip !== "") {
      writer.uint32(18).string(message.ip);
    }
    if (message.outgoingSequenceNum !== 0) {
      writer.uint32(24).uint32(message.outgoingSequenceNum);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStationConnection {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStationConnection();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.alive = reader.bool();
          break;
        case 2:
          message.ip = reader.string();
          break;
        case 3:
          message.outgoingSequenceNum = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStationConnection {
    return {
      alive: isSet(object.alive) ? Boolean(object.alive) : false,
      ip: isSet(object.ip) ? String(object.ip) : "",
      outgoingSequenceNum: isSet(object.outgoingSequenceNum) ? Number(object.outgoingSequenceNum) : 0,
    };
  },

  toJSON(message: DriverStationConnection): unknown {
    const obj: any = {};
    message.alive !== undefined && (obj.alive = message.alive);
    message.ip !== undefined && (obj.ip = message.ip);
    message.outgoingSequenceNum !== undefined && (obj.outgoingSequenceNum = Math.round(message.outgoingSequenceNum));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStationConnection>, I>>(object: I): DriverStationConnection {
    const message = createBaseDriverStationConnection();
    message.alive = object.alive ?? false;
    message.ip = object.ip ?? "";
    message.outgoingSequenceNum = object.outgoingSequenceNum ?? 0;
    return message;
  },
};

function createBaseDriverStationConfirmedState(): DriverStationConfirmedState {
  return {
    isEmergencyStopped: false,
    robotCommunicationsActive: false,
    canPingRadio: false,
    canPingRio: false,
    isEnabled: false,
    mode: 0,
    teamNumber: 0,
    batteryVoltage: 0,
  };
}

export const DriverStationConfirmedState = {
  encode(message: DriverStationConfirmedState, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.isEmergencyStopped === true) {
      writer.uint32(8).bool(message.isEmergencyStopped);
    }
    if (message.robotCommunicationsActive === true) {
      writer.uint32(16).bool(message.robotCommunicationsActive);
    }
    if (message.canPingRadio === true) {
      writer.uint32(24).bool(message.canPingRadio);
    }
    if (message.canPingRio === true) {
      writer.uint32(32).bool(message.canPingRio);
    }
    if (message.isEnabled === true) {
      writer.uint32(40).bool(message.isEnabled);
    }
    if (message.mode !== 0) {
      writer.uint32(48).int32(message.mode);
    }
    if (message.teamNumber !== 0) {
      writer.uint32(56).uint32(message.teamNumber);
    }
    if (message.batteryVoltage !== 0) {
      writer.uint32(69).float(message.batteryVoltage);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DriverStationConfirmedState {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDriverStationConfirmedState();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.isEmergencyStopped = reader.bool();
          break;
        case 2:
          message.robotCommunicationsActive = reader.bool();
          break;
        case 3:
          message.canPingRadio = reader.bool();
          break;
        case 4:
          message.canPingRio = reader.bool();
          break;
        case 5:
          message.isEnabled = reader.bool();
          break;
        case 6:
          message.mode = reader.int32() as any;
          break;
        case 7:
          message.teamNumber = reader.uint32();
          break;
        case 8:
          message.batteryVoltage = reader.float();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DriverStationConfirmedState {
    return {
      isEmergencyStopped: isSet(object.isEmergencyStopped) ? Boolean(object.isEmergencyStopped) : false,
      robotCommunicationsActive: isSet(object.robotCommunicationsActive)
        ? Boolean(object.robotCommunicationsActive)
        : false,
      canPingRadio: isSet(object.canPingRadio) ? Boolean(object.canPingRadio) : false,
      canPingRio: isSet(object.canPingRio) ? Boolean(object.canPingRio) : false,
      isEnabled: isSet(object.isEnabled) ? Boolean(object.isEnabled) : false,
      mode: isSet(object.mode) ? modeFromJSON(object.mode) : 0,
      teamNumber: isSet(object.teamNumber) ? Number(object.teamNumber) : 0,
      batteryVoltage: isSet(object.batteryVoltage) ? Number(object.batteryVoltage) : 0,
    };
  },

  toJSON(message: DriverStationConfirmedState): unknown {
    const obj: any = {};
    message.isEmergencyStopped !== undefined && (obj.isEmergencyStopped = message.isEmergencyStopped);
    message.robotCommunicationsActive !== undefined &&
      (obj.robotCommunicationsActive = message.robotCommunicationsActive);
    message.canPingRadio !== undefined && (obj.canPingRadio = message.canPingRadio);
    message.canPingRio !== undefined && (obj.canPingRio = message.canPingRio);
    message.isEnabled !== undefined && (obj.isEnabled = message.isEnabled);
    message.mode !== undefined && (obj.mode = modeToJSON(message.mode));
    message.teamNumber !== undefined && (obj.teamNumber = Math.round(message.teamNumber));
    message.batteryVoltage !== undefined && (obj.batteryVoltage = message.batteryVoltage);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DriverStationConfirmedState>, I>>(object: I): DriverStationConfirmedState {
    const message = createBaseDriverStationConfirmedState();
    message.isEmergencyStopped = object.isEmergencyStopped ?? false;
    message.robotCommunicationsActive = object.robotCommunicationsActive ?? false;
    message.canPingRadio = object.canPingRadio ?? false;
    message.canPingRio = object.canPingRio ?? false;
    message.isEnabled = object.isEnabled ?? false;
    message.mode = object.mode ?? 0;
    message.teamNumber = object.teamNumber ?? 0;
    message.batteryVoltage = object.batteryVoltage ?? 0;
    return message;
  },
};

function createBaseEnablerQuery(): EnablerQuery {
  return { id: "" };
}

export const EnablerQuery = {
  encode(message: EnablerQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EnablerQuery {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEnablerQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EnablerQuery {
    return { id: isSet(object.id) ? String(object.id) : "" };
  },

  toJSON(message: EnablerQuery): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<EnablerQuery>, I>>(object: I): EnablerQuery {
    const message = createBaseEnablerQuery();
    message.id = object.id ?? "";
    return message;
  },
};

function createBaseEnablerConfig(): EnablerConfig {
  return { id: "", name: "", allEnabler: undefined, teamNumberEnabler: undefined, allianceStationEnabler: undefined };
}

export const EnablerConfig = {
  encode(message: EnablerConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.allEnabler !== undefined) {
      AllEnablerConfig.encode(message.allEnabler, writer.uint32(26).fork()).ldelim();
    }
    if (message.teamNumberEnabler !== undefined) {
      TeamNumberEnablerConfig.encode(message.teamNumberEnabler, writer.uint32(34).fork()).ldelim();
    }
    if (message.allianceStationEnabler !== undefined) {
      AllianceStationEnablerConfig.encode(message.allianceStationEnabler, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EnablerConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEnablerConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.allEnabler = AllEnablerConfig.decode(reader, reader.uint32());
          break;
        case 4:
          message.teamNumberEnabler = TeamNumberEnablerConfig.decode(reader, reader.uint32());
          break;
        case 5:
          message.allianceStationEnabler = AllianceStationEnablerConfig.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EnablerConfig {
    return {
      id: isSet(object.id) ? String(object.id) : "",
      name: isSet(object.name) ? String(object.name) : "",
      allEnabler: isSet(object.allEnabler) ? AllEnablerConfig.fromJSON(object.allEnabler) : undefined,
      teamNumberEnabler: isSet(object.teamNumberEnabler)
        ? TeamNumberEnablerConfig.fromJSON(object.teamNumberEnabler)
        : undefined,
      allianceStationEnabler: isSet(object.allianceStationEnabler)
        ? AllianceStationEnablerConfig.fromJSON(object.allianceStationEnabler)
        : undefined,
    };
  },

  toJSON(message: EnablerConfig): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    message.allEnabler !== undefined &&
      (obj.allEnabler = message.allEnabler ? AllEnablerConfig.toJSON(message.allEnabler) : undefined);
    message.teamNumberEnabler !== undefined && (obj.teamNumberEnabler = message.teamNumberEnabler
      ? TeamNumberEnablerConfig.toJSON(message.teamNumberEnabler)
      : undefined);
    message.allianceStationEnabler !== undefined && (obj.allianceStationEnabler = message.allianceStationEnabler
      ? AllianceStationEnablerConfig.toJSON(message.allianceStationEnabler)
      : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<EnablerConfig>, I>>(object: I): EnablerConfig {
    const message = createBaseEnablerConfig();
    message.id = object.id ?? "";
    message.name = object.name ?? "";
    message.allEnabler = (object.allEnabler !== undefined && object.allEnabler !== null)
      ? AllEnablerConfig.fromPartial(object.allEnabler)
      : undefined;
    message.teamNumberEnabler = (object.teamNumberEnabler !== undefined && object.teamNumberEnabler !== null)
      ? TeamNumberEnablerConfig.fromPartial(object.teamNumberEnabler)
      : undefined;
    message.allianceStationEnabler =
      (object.allianceStationEnabler !== undefined && object.allianceStationEnabler !== null)
        ? AllianceStationEnablerConfig.fromPartial(object.allianceStationEnabler)
        : undefined;
    return message;
  },
};

function createBaseAllEnablerConfig(): AllEnablerConfig {
  return { active: false };
}

export const AllEnablerConfig = {
  encode(message: AllEnablerConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.active === true) {
      writer.uint32(8).bool(message.active);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AllEnablerConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAllEnablerConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.active = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AllEnablerConfig {
    return { active: isSet(object.active) ? Boolean(object.active) : false };
  },

  toJSON(message: AllEnablerConfig): unknown {
    const obj: any = {};
    message.active !== undefined && (obj.active = message.active);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AllEnablerConfig>, I>>(object: I): AllEnablerConfig {
    const message = createBaseAllEnablerConfig();
    message.active = object.active ?? false;
    return message;
  },
};

function createBaseTeamNumberEnablerConfig(): TeamNumberEnablerConfig {
  return { approvedTeamNumbers: [] };
}

export const TeamNumberEnablerConfig = {
  encode(message: TeamNumberEnablerConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.approvedTeamNumbers) {
      writer.uint32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TeamNumberEnablerConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTeamNumberEnablerConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.approvedTeamNumbers.push(reader.uint32());
            }
          } else {
            message.approvedTeamNumbers.push(reader.uint32());
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): TeamNumberEnablerConfig {
    return {
      approvedTeamNumbers: Array.isArray(object?.approvedTeamNumbers)
        ? object.approvedTeamNumbers.map((e: any) => Number(e))
        : [],
    };
  },

  toJSON(message: TeamNumberEnablerConfig): unknown {
    const obj: any = {};
    if (message.approvedTeamNumbers) {
      obj.approvedTeamNumbers = message.approvedTeamNumbers.map((e) => Math.round(e));
    } else {
      obj.approvedTeamNumbers = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<TeamNumberEnablerConfig>, I>>(object: I): TeamNumberEnablerConfig {
    const message = createBaseTeamNumberEnablerConfig();
    message.approvedTeamNumbers = object.approvedTeamNumbers?.map((e) => e) || [];
    return message;
  },
};

function createBaseAllianceStationEnablerConfig(): AllianceStationEnablerConfig {
  return { approvedStations: [] };
}

export const AllianceStationEnablerConfig = {
  encode(message: AllianceStationEnablerConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.approvedStations) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AllianceStationEnablerConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAllianceStationEnablerConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.approvedStations.push(reader.int32() as any);
            }
          } else {
            message.approvedStations.push(reader.int32() as any);
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AllianceStationEnablerConfig {
    return {
      approvedStations: Array.isArray(object?.approvedStations)
        ? object.approvedStations.map((e: any) => allianceStationFromJSON(e))
        : [],
    };
  },

  toJSON(message: AllianceStationEnablerConfig): unknown {
    const obj: any = {};
    if (message.approvedStations) {
      obj.approvedStations = message.approvedStations.map((e) => allianceStationToJSON(e));
    } else {
      obj.approvedStations = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AllianceStationEnablerConfig>, I>>(object: I): AllianceStationEnablerConfig {
    const message = createBaseAllianceStationEnablerConfig();
    message.approvedStations = object.approvedStations?.map((e) => e) || [];
    return message;
  },
};

function createBaseEstopperQuery(): EstopperQuery {
  return { id: "" };
}

export const EstopperQuery = {
  encode(message: EstopperQuery, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EstopperQuery {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEstopperQuery();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EstopperQuery {
    return { id: isSet(object.id) ? String(object.id) : "" };
  },

  toJSON(message: EstopperQuery): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<EstopperQuery>, I>>(object: I): EstopperQuery {
    const message = createBaseEstopperQuery();
    message.id = object.id ?? "";
    return message;
  },
};

function createBaseEstopperConfig(): EstopperConfig {
  return {
    id: "",
    name: "",
    allEstopper: undefined,
    teamNumberEstopper: undefined,
    allianceStationEstopper: undefined,
  };
}

export const EstopperConfig = {
  encode(message: EstopperConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.allEstopper !== undefined) {
      AllEstopperConfig.encode(message.allEstopper, writer.uint32(26).fork()).ldelim();
    }
    if (message.teamNumberEstopper !== undefined) {
      TeamNumberEstopperConfig.encode(message.teamNumberEstopper, writer.uint32(34).fork()).ldelim();
    }
    if (message.allianceStationEstopper !== undefined) {
      AllianceStationEstopperConfig.encode(message.allianceStationEstopper, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EstopperConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEstopperConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.allEstopper = AllEstopperConfig.decode(reader, reader.uint32());
          break;
        case 4:
          message.teamNumberEstopper = TeamNumberEstopperConfig.decode(reader, reader.uint32());
          break;
        case 5:
          message.allianceStationEstopper = AllianceStationEstopperConfig.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EstopperConfig {
    return {
      id: isSet(object.id) ? String(object.id) : "",
      name: isSet(object.name) ? String(object.name) : "",
      allEstopper: isSet(object.allEstopper) ? AllEstopperConfig.fromJSON(object.allEstopper) : undefined,
      teamNumberEstopper: isSet(object.teamNumberEstopper)
        ? TeamNumberEstopperConfig.fromJSON(object.teamNumberEstopper)
        : undefined,
      allianceStationEstopper: isSet(object.allianceStationEstopper)
        ? AllianceStationEstopperConfig.fromJSON(object.allianceStationEstopper)
        : undefined,
    };
  },

  toJSON(message: EstopperConfig): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    message.allEstopper !== undefined &&
      (obj.allEstopper = message.allEstopper ? AllEstopperConfig.toJSON(message.allEstopper) : undefined);
    message.teamNumberEstopper !== undefined && (obj.teamNumberEstopper = message.teamNumberEstopper
      ? TeamNumberEstopperConfig.toJSON(message.teamNumberEstopper)
      : undefined);
    message.allianceStationEstopper !== undefined && (obj.allianceStationEstopper = message.allianceStationEstopper
      ? AllianceStationEstopperConfig.toJSON(message.allianceStationEstopper)
      : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<EstopperConfig>, I>>(object: I): EstopperConfig {
    const message = createBaseEstopperConfig();
    message.id = object.id ?? "";
    message.name = object.name ?? "";
    message.allEstopper = (object.allEstopper !== undefined && object.allEstopper !== null)
      ? AllEstopperConfig.fromPartial(object.allEstopper)
      : undefined;
    message.teamNumberEstopper = (object.teamNumberEstopper !== undefined && object.teamNumberEstopper !== null)
      ? TeamNumberEstopperConfig.fromPartial(object.teamNumberEstopper)
      : undefined;
    message.allianceStationEstopper =
      (object.allianceStationEstopper !== undefined && object.allianceStationEstopper !== null)
        ? AllianceStationEstopperConfig.fromPartial(object.allianceStationEstopper)
        : undefined;
    return message;
  },
};

function createBaseAllEstopperConfig(): AllEstopperConfig {
  return { active: false };
}

export const AllEstopperConfig = {
  encode(message: AllEstopperConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.active === true) {
      writer.uint32(8).bool(message.active);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AllEstopperConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAllEstopperConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.active = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AllEstopperConfig {
    return { active: isSet(object.active) ? Boolean(object.active) : false };
  },

  toJSON(message: AllEstopperConfig): unknown {
    const obj: any = {};
    message.active !== undefined && (obj.active = message.active);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AllEstopperConfig>, I>>(object: I): AllEstopperConfig {
    const message = createBaseAllEstopperConfig();
    message.active = object.active ?? false;
    return message;
  },
};

function createBaseTeamNumberEstopperConfig(): TeamNumberEstopperConfig {
  return { estoppedTeamNumbers: [] };
}

export const TeamNumberEstopperConfig = {
  encode(message: TeamNumberEstopperConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.estoppedTeamNumbers) {
      writer.uint32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TeamNumberEstopperConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTeamNumberEstopperConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.estoppedTeamNumbers.push(reader.uint32());
            }
          } else {
            message.estoppedTeamNumbers.push(reader.uint32());
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): TeamNumberEstopperConfig {
    return {
      estoppedTeamNumbers: Array.isArray(object?.estoppedTeamNumbers)
        ? object.estoppedTeamNumbers.map((e: any) => Number(e))
        : [],
    };
  },

  toJSON(message: TeamNumberEstopperConfig): unknown {
    const obj: any = {};
    if (message.estoppedTeamNumbers) {
      obj.estoppedTeamNumbers = message.estoppedTeamNumbers.map((e) => Math.round(e));
    } else {
      obj.estoppedTeamNumbers = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<TeamNumberEstopperConfig>, I>>(object: I): TeamNumberEstopperConfig {
    const message = createBaseTeamNumberEstopperConfig();
    message.estoppedTeamNumbers = object.estoppedTeamNumbers?.map((e) => e) || [];
    return message;
  },
};

function createBaseAllianceStationEstopperConfig(): AllianceStationEstopperConfig {
  return { estoppedStations: [] };
}

export const AllianceStationEstopperConfig = {
  encode(message: AllianceStationEstopperConfig, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.estoppedStations) {
      writer.int32(v);
    }
    writer.ldelim();
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AllianceStationEstopperConfig {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAllianceStationEstopperConfig();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.estoppedStations.push(reader.int32() as any);
            }
          } else {
            message.estoppedStations.push(reader.int32() as any);
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AllianceStationEstopperConfig {
    return {
      estoppedStations: Array.isArray(object?.estoppedStations)
        ? object.estoppedStations.map((e: any) => allianceStationFromJSON(e))
        : [],
    };
  },

  toJSON(message: AllianceStationEstopperConfig): unknown {
    const obj: any = {};
    if (message.estoppedStations) {
      obj.estoppedStations = message.estoppedStations.map((e) => allianceStationToJSON(e));
    } else {
      obj.estoppedStations = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<AllianceStationEstopperConfig>, I>>(
    object: I,
  ): AllianceStationEstopperConfig {
    const message = createBaseAllianceStationEstopperConfig();
    message.estoppedStations = object.estoppedStations?.map((e) => e) || [];
    return message;
  },
};

function createBaseDiffTimer(): DiffTimer {
  return { startedAt: 0, timeRemaining: 0 };
}

export const DiffTimer = {
  encode(message: DiffTimer, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.startedAt !== 0) {
      writer.uint32(8).uint64(message.startedAt);
    }
    if (message.timeRemaining !== 0) {
      writer.uint32(16).uint64(message.timeRemaining);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DiffTimer {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDiffTimer();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.startedAt = longToNumber(reader.uint64() as Long);
          break;
        case 2:
          message.timeRemaining = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DiffTimer {
    return {
      startedAt: isSet(object.startedAt) ? Number(object.startedAt) : 0,
      timeRemaining: isSet(object.timeRemaining) ? Number(object.timeRemaining) : 0,
    };
  },

  toJSON(message: DiffTimer): unknown {
    const obj: any = {};
    message.startedAt !== undefined && (obj.startedAt = Math.round(message.startedAt));
    message.timeRemaining !== undefined && (obj.timeRemaining = Math.round(message.timeRemaining));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DiffTimer>, I>>(object: I): DiffTimer {
    const message = createBaseDiffTimer();
    message.startedAt = object.startedAt ?? 0;
    message.timeRemaining = object.timeRemaining ?? 0;
    return message;
  },
};

export type PluginAPIService = typeof PluginAPIService;
export const PluginAPIService = {
  /** Plugin Registration Functions */
  registerPlugin: {
    path: "/plugin.PluginAPI/RegisterPlugin",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: PluginRegistrationRequest) =>
      Buffer.from(PluginRegistrationRequest.encode(value).finish()),
    requestDeserialize: (value: Buffer) => PluginRegistrationRequest.decode(value),
    responseSerialize: (value: PluginRegistrationResponse) =>
      Buffer.from(PluginRegistrationResponse.encode(value).finish()),
    responseDeserialize: (value: Buffer) => PluginRegistrationResponse.decode(value),
  },
  /** Plugin Heartbeat */
  heartbeat: {
    path: "/plugin.PluginAPI/Heartbeat",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  jsonRPCPublish: {
    path: "/plugin.PluginAPI/JsonRPCPublish",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: JSONRpcMessage) => Buffer.from(JSONRpcMessage.encode(value).finish()),
    requestDeserialize: (value: Buffer) => JSONRpcMessage.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  jsonRPCSubscribe: {
    path: "/plugin.PluginAPI/JsonRPCSubscribe",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: JSONRpcMessage) => Buffer.from(JSONRpcMessage.encode(value).finish()),
    responseDeserialize: (value: Buffer) => JSONRpcMessage.decode(value),
  },
  /** Network Configuration Functions */
  updateDriverStationExpectedIP: {
    path: "/plugin.PluginAPI/UpdateDriverStationExpectedIP",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: DriverStationUpdateExpectedIP) =>
      Buffer.from(DriverStationUpdateExpectedIP.encode(value).finish()),
    requestDeserialize: (value: Buffer) => DriverStationUpdateExpectedIP.decode(value),
    responseSerialize: (value: DriverStation) => Buffer.from(DriverStation.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStation.decode(value),
  },
  /** Field Functions */
  onFieldStateUpdate: {
    path: "/plugin.PluginAPI/OnFieldStateUpdate",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: FieldState) => Buffer.from(FieldState.encode(value).finish()),
    responseDeserialize: (value: Buffer) => FieldState.decode(value),
  },
  onFieldTerminate: {
    path: "/plugin.PluginAPI/OnFieldTerminate",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: FieldState) => Buffer.from(FieldState.encode(value).finish()),
    responseDeserialize: (value: Buffer) => FieldState.decode(value),
  },
  getFieldState: {
    path: "/plugin.PluginAPI/GetFieldState",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: FieldState) => Buffer.from(FieldState.encode(value).finish()),
    responseDeserialize: (value: Buffer) => FieldState.decode(value),
  },
  configureField: {
    path: "/plugin.PluginAPI/ConfigureField",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: FieldConfiguration) => Buffer.from(FieldConfiguration.encode(value).finish()),
    requestDeserialize: (value: Buffer) => FieldConfiguration.decode(value),
    responseSerialize: (value: FieldState) => Buffer.from(FieldState.encode(value).finish()),
    responseDeserialize: (value: Buffer) => FieldState.decode(value),
  },
  updateFieldTimer: {
    path: "/plugin.PluginAPI/UpdateFieldTimer",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: FieldTimerUpdate) => Buffer.from(FieldTimerUpdate.encode(value).finish()),
    requestDeserialize: (value: Buffer) => FieldTimerUpdate.decode(value),
    responseSerialize: (value: FieldState) => Buffer.from(FieldState.encode(value).finish()),
    responseDeserialize: (value: Buffer) => FieldState.decode(value),
  },
  updateEnabler: {
    path: "/plugin.PluginAPI/UpdateEnabler",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: EnablerConfig) => Buffer.from(EnablerConfig.encode(value).finish()),
    requestDeserialize: (value: Buffer) => EnablerConfig.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  removeEnabler: {
    path: "/plugin.PluginAPI/RemoveEnabler",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: EnablerQuery) => Buffer.from(EnablerQuery.encode(value).finish()),
    requestDeserialize: (value: Buffer) => EnablerQuery.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  updateEstopper: {
    path: "/plugin.PluginAPI/UpdateEstopper",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: EstopperConfig) => Buffer.from(EstopperConfig.encode(value).finish()),
    requestDeserialize: (value: Buffer) => EstopperConfig.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  removeEstopper: {
    path: "/plugin.PluginAPI/RemoveEstopper",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: EstopperQuery) => Buffer.from(EstopperQuery.encode(value).finish()),
    requestDeserialize: (value: Buffer) => EstopperQuery.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  onDriverStationCreate: {
    path: "/plugin.PluginAPI/OnDriverStationCreate",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: DriverStation) => Buffer.from(DriverStation.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStation.decode(value),
  },
  onDriverStationUpdate: {
    path: "/plugin.PluginAPI/OnDriverStationUpdate",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: DriverStationQuery) => Buffer.from(DriverStationQuery.encode(value).finish()),
    requestDeserialize: (value: Buffer) => DriverStationQuery.decode(value),
    responseSerialize: (value: DriverStation) => Buffer.from(DriverStation.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStation.decode(value),
  },
  onDriverStationDelete: {
    path: "/plugin.PluginAPI/OnDriverStationDelete",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: DriverStation) => Buffer.from(DriverStation.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStation.decode(value),
  },
  getDriverStations: {
    path: "/plugin.PluginAPI/GetDriverStations",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: DriverStations) => Buffer.from(DriverStations.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStations.decode(value),
  },
  getDriverStation: {
    path: "/plugin.PluginAPI/GetDriverStation",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: DriverStationQuery) => Buffer.from(DriverStationQuery.encode(value).finish()),
    requestDeserialize: (value: Buffer) => DriverStationQuery.decode(value),
    responseSerialize: (value: DriverStation) => Buffer.from(DriverStation.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStation.decode(value),
  },
  addDriverStation: {
    path: "/plugin.PluginAPI/AddDriverStation",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: DriverStationParams) => Buffer.from(DriverStationParams.encode(value).finish()),
    requestDeserialize: (value: Buffer) => DriverStationParams.decode(value),
    responseSerialize: (value: DriverStation) => Buffer.from(DriverStation.encode(value).finish()),
    responseDeserialize: (value: Buffer) => DriverStation.decode(value),
  },
  deleteDriverStation: {
    path: "/plugin.PluginAPI/DeleteDriverStation",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: DriverStationParams) => Buffer.from(DriverStationParams.encode(value).finish()),
    requestDeserialize: (value: Buffer) => DriverStationParams.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
  updateDriverStationMode: {
    path: "/plugin.PluginAPI/UpdateDriverStationMode",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: DriverStationUpdateMode) => Buffer.from(DriverStationUpdateMode.encode(value).finish()),
    requestDeserialize: (value: Buffer) => DriverStationUpdateMode.decode(value),
    responseSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    responseDeserialize: (value: Buffer) => Empty.decode(value),
  },
} as const;

export interface PluginAPIServer extends UntypedServiceImplementation {
  /** Plugin Registration Functions */
  registerPlugin: handleUnaryCall<PluginRegistrationRequest, PluginRegistrationResponse>;
  /** Plugin Heartbeat */
  heartbeat: handleUnaryCall<Empty, Empty>;
  jsonRPCPublish: handleUnaryCall<JSONRpcMessage, Empty>;
  jsonRPCSubscribe: handleServerStreamingCall<Empty, JSONRpcMessage>;
  /** Network Configuration Functions */
  updateDriverStationExpectedIP: handleUnaryCall<DriverStationUpdateExpectedIP, DriverStation>;
  /** Field Functions */
  onFieldStateUpdate: handleServerStreamingCall<Empty, FieldState>;
  onFieldTerminate: handleServerStreamingCall<Empty, FieldState>;
  getFieldState: handleUnaryCall<Empty, FieldState>;
  configureField: handleUnaryCall<FieldConfiguration, FieldState>;
  updateFieldTimer: handleUnaryCall<FieldTimerUpdate, FieldState>;
  updateEnabler: handleUnaryCall<EnablerConfig, Empty>;
  removeEnabler: handleUnaryCall<EnablerQuery, Empty>;
  updateEstopper: handleUnaryCall<EstopperConfig, Empty>;
  removeEstopper: handleUnaryCall<EstopperQuery, Empty>;
  onDriverStationCreate: handleServerStreamingCall<Empty, DriverStation>;
  onDriverStationUpdate: handleServerStreamingCall<DriverStationQuery, DriverStation>;
  onDriverStationDelete: handleServerStreamingCall<Empty, DriverStation>;
  getDriverStations: handleUnaryCall<Empty, DriverStations>;
  getDriverStation: handleUnaryCall<DriverStationQuery, DriverStation>;
  addDriverStation: handleUnaryCall<DriverStationParams, DriverStation>;
  deleteDriverStation: handleUnaryCall<DriverStationParams, Empty>;
  updateDriverStationMode: handleUnaryCall<DriverStationUpdateMode, Empty>;
}

export interface PluginAPIClient extends Client {
  /** Plugin Registration Functions */
  registerPlugin(
    request: PluginRegistrationRequest,
    callback: (error: ServiceError | null, response: PluginRegistrationResponse) => void,
  ): ClientUnaryCall;
  registerPlugin(
    request: PluginRegistrationRequest,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: PluginRegistrationResponse) => void,
  ): ClientUnaryCall;
  registerPlugin(
    request: PluginRegistrationRequest,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: PluginRegistrationResponse) => void,
  ): ClientUnaryCall;
  /** Plugin Heartbeat */
  heartbeat(request: Empty, callback: (error: ServiceError | null, response: Empty) => void): ClientUnaryCall;
  heartbeat(
    request: Empty,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  heartbeat(
    request: Empty,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  jsonRPCPublish(
    request: JSONRpcMessage,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  jsonRPCPublish(
    request: JSONRpcMessage,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  jsonRPCPublish(
    request: JSONRpcMessage,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  jsonRPCSubscribe(request: Empty, options?: Partial<CallOptions>): ClientReadableStream<JSONRpcMessage>;
  jsonRPCSubscribe(
    request: Empty,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<JSONRpcMessage>;
  /** Network Configuration Functions */
  updateDriverStationExpectedIP(
    request: DriverStationUpdateExpectedIP,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  updateDriverStationExpectedIP(
    request: DriverStationUpdateExpectedIP,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  updateDriverStationExpectedIP(
    request: DriverStationUpdateExpectedIP,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  /** Field Functions */
  onFieldStateUpdate(request: Empty, options?: Partial<CallOptions>): ClientReadableStream<FieldState>;
  onFieldStateUpdate(
    request: Empty,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<FieldState>;
  onFieldTerminate(request: Empty, options?: Partial<CallOptions>): ClientReadableStream<FieldState>;
  onFieldTerminate(
    request: Empty,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<FieldState>;
  getFieldState(request: Empty, callback: (error: ServiceError | null, response: FieldState) => void): ClientUnaryCall;
  getFieldState(
    request: Empty,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  getFieldState(
    request: Empty,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  configureField(
    request: FieldConfiguration,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  configureField(
    request: FieldConfiguration,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  configureField(
    request: FieldConfiguration,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  updateFieldTimer(
    request: FieldTimerUpdate,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  updateFieldTimer(
    request: FieldTimerUpdate,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  updateFieldTimer(
    request: FieldTimerUpdate,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: FieldState) => void,
  ): ClientUnaryCall;
  updateEnabler(
    request: EnablerConfig,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateEnabler(
    request: EnablerConfig,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateEnabler(
    request: EnablerConfig,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  removeEnabler(
    request: EnablerQuery,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  removeEnabler(
    request: EnablerQuery,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  removeEnabler(
    request: EnablerQuery,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateEstopper(
    request: EstopperConfig,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateEstopper(
    request: EstopperConfig,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateEstopper(
    request: EstopperConfig,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  removeEstopper(
    request: EstopperQuery,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  removeEstopper(
    request: EstopperQuery,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  removeEstopper(
    request: EstopperQuery,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  onDriverStationCreate(request: Empty, options?: Partial<CallOptions>): ClientReadableStream<DriverStation>;
  onDriverStationCreate(
    request: Empty,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<DriverStation>;
  onDriverStationUpdate(
    request: DriverStationQuery,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<DriverStation>;
  onDriverStationUpdate(
    request: DriverStationQuery,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<DriverStation>;
  onDriverStationDelete(request: Empty, options?: Partial<CallOptions>): ClientReadableStream<DriverStation>;
  onDriverStationDelete(
    request: Empty,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<DriverStation>;
  getDriverStations(
    request: Empty,
    callback: (error: ServiceError | null, response: DriverStations) => void,
  ): ClientUnaryCall;
  getDriverStations(
    request: Empty,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: DriverStations) => void,
  ): ClientUnaryCall;
  getDriverStations(
    request: Empty,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: DriverStations) => void,
  ): ClientUnaryCall;
  getDriverStation(
    request: DriverStationQuery,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  getDriverStation(
    request: DriverStationQuery,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  getDriverStation(
    request: DriverStationQuery,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  addDriverStation(
    request: DriverStationParams,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  addDriverStation(
    request: DriverStationParams,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  addDriverStation(
    request: DriverStationParams,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: DriverStation) => void,
  ): ClientUnaryCall;
  deleteDriverStation(
    request: DriverStationParams,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  deleteDriverStation(
    request: DriverStationParams,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  deleteDriverStation(
    request: DriverStationParams,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateDriverStationMode(
    request: DriverStationUpdateMode,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateDriverStationMode(
    request: DriverStationUpdateMode,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
  updateDriverStationMode(
    request: DriverStationUpdateMode,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: Empty) => void,
  ): ClientUnaryCall;
}

export const PluginAPIClient = makeGenericClientConstructor(PluginAPIService, "plugin.PluginAPI") as unknown as {
  new (address: string, credentials: ChannelCredentials, options?: Partial<ChannelOptions>): PluginAPIClient;
  service: typeof PluginAPIService;
};

declare var self: any | undefined;
declare var window: any | undefined;
declare var global: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") {
    return globalThis;
  }
  if (typeof self !== "undefined") {
    return self;
  }
  if (typeof window !== "undefined") {
    return window;
  }
  if (typeof global !== "undefined") {
    return global;
  }
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

type DeepPartial<T> = T extends Builtin ? T
  : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
