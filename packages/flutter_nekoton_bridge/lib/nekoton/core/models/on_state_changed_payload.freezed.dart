// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'on_state_changed_payload.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

OnStateChangedPayload _$OnStateChangedPayloadFromJson(
    Map<String, dynamic> json) {
  return _OnStateChangedPayload.fromJson(json);
}

/// @nodoc
mixin _$OnStateChangedPayload {
  ContractState get newState => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $OnStateChangedPayloadCopyWith<OnStateChangedPayload> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OnStateChangedPayloadCopyWith<$Res> {
  factory $OnStateChangedPayloadCopyWith(OnStateChangedPayload value,
          $Res Function(OnStateChangedPayload) then) =
      _$OnStateChangedPayloadCopyWithImpl<$Res, OnStateChangedPayload>;
  @useResult
  $Res call({ContractState newState});

  $ContractStateCopyWith<$Res> get newState;
}

/// @nodoc
class _$OnStateChangedPayloadCopyWithImpl<$Res,
        $Val extends OnStateChangedPayload>
    implements $OnStateChangedPayloadCopyWith<$Res> {
  _$OnStateChangedPayloadCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? newState = null,
  }) {
    return _then(_value.copyWith(
      newState: null == newState
          ? _value.newState
          : newState // ignore: cast_nullable_to_non_nullable
              as ContractState,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $ContractStateCopyWith<$Res> get newState {
    return $ContractStateCopyWith<$Res>(_value.newState, (value) {
      return _then(_value.copyWith(newState: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$OnStateChangedPayloadImplCopyWith<$Res>
    implements $OnStateChangedPayloadCopyWith<$Res> {
  factory _$$OnStateChangedPayloadImplCopyWith(
          _$OnStateChangedPayloadImpl value,
          $Res Function(_$OnStateChangedPayloadImpl) then) =
      __$$OnStateChangedPayloadImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({ContractState newState});

  @override
  $ContractStateCopyWith<$Res> get newState;
}

/// @nodoc
class __$$OnStateChangedPayloadImplCopyWithImpl<$Res>
    extends _$OnStateChangedPayloadCopyWithImpl<$Res,
        _$OnStateChangedPayloadImpl>
    implements _$$OnStateChangedPayloadImplCopyWith<$Res> {
  __$$OnStateChangedPayloadImplCopyWithImpl(_$OnStateChangedPayloadImpl _value,
      $Res Function(_$OnStateChangedPayloadImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? newState = null,
  }) {
    return _then(_$OnStateChangedPayloadImpl(
      newState: null == newState
          ? _value.newState
          : newState // ignore: cast_nullable_to_non_nullable
              as ContractState,
    ));
  }
}

/// @nodoc

@JsonSerializable(fieldRename: FieldRename.snake, explicitToJson: true)
class _$OnStateChangedPayloadImpl implements _OnStateChangedPayload {
  const _$OnStateChangedPayloadImpl({required this.newState});

  factory _$OnStateChangedPayloadImpl.fromJson(Map<String, dynamic> json) =>
      _$$OnStateChangedPayloadImplFromJson(json);

  @override
  final ContractState newState;

  @override
  String toString() {
    return 'OnStateChangedPayload(newState: $newState)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$OnStateChangedPayloadImpl &&
            (identical(other.newState, newState) ||
                other.newState == newState));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, newState);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$OnStateChangedPayloadImplCopyWith<_$OnStateChangedPayloadImpl>
      get copyWith => __$$OnStateChangedPayloadImplCopyWithImpl<
          _$OnStateChangedPayloadImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$OnStateChangedPayloadImplToJson(
      this,
    );
  }
}

abstract class _OnStateChangedPayload implements OnStateChangedPayload {
  const factory _OnStateChangedPayload(
      {required final ContractState newState}) = _$OnStateChangedPayloadImpl;

  factory _OnStateChangedPayload.fromJson(Map<String, dynamic> json) =
      _$OnStateChangedPayloadImpl.fromJson;

  @override
  ContractState get newState;
  @override
  @JsonKey(ignore: true)
  _$$OnStateChangedPayloadImplCopyWith<_$OnStateChangedPayloadImpl>
      get copyWith => throw _privateConstructorUsedError;
}
