// home for giant enums which will clutter the rest of the codebase

// tag code, see fiff/tags.tsv
// is currently missing the MNE-specific tags.
// carries a catchall Code(i32) variant to hold any codes not recognised here
#[derive(Debug, Default)]
pub enum Kind {
    #[default]
    FileId,
    DirPointer,
    Dir,
    BlockId,
    BlockStart,
    BlockEnd,
    FreeList,
    FreeBlock,
    Nop,
    ParentFileId,
    ParentBlockId,
    BlockName,
    BlockVersion,
    Creator,
    Modifier,
    RefRole,
    RefFileId,
    RefFileNum,
    RefFileName,
    RefBlockId,
    DacqPars,
    DacqStim,
    DeviceType,
    DeviceModel,
    DeviceSerial,
    DeviceSite,
    HeLevelRaw,
    HeliumLevel,
    OrigFileGuid,
    UtcOffset,
    Nchan,
    Sfreq,
    DataPack,
    ChInfo,
    MeasDate,
    Subject,
    Description,
    Nave,
    FirstSample,
    LastSample,
    AspectKind,
    RefEvent,
    Experimenter,
    DigPoint,
    ChPosVec,
    HpiSlopes,
    HpiNcoil,
    ReqEvent,
    ReqLimit,
    Lowpass,
    BadChs,
    ArtefRemoval,
    CoordTrans,
    Highpass,
    ChCalsVec,
    HpiBadChs,
    HpiCorrCoeff,
    EventComment,
    NoSamples,
    FirstTime,
    SubaveSize,
    SubaveFirst,
    Name,
    DigString,
    LineFreq,
    HpiCoilFreq,
    SignalChannel,
    HpiCoilMoments,
    HpiFitGoodness,
    HpiFitAccept,
    HpiFitGoodLimit,
    HpiFitDistLimit,
    HpiCoilNo,
    HpiCoilsUsed,
    HpiDigitizationOrder,
    ChScanNo,
    ChLogicalNo,
    ChKind,
    ChRange,
    ChCal,
    ChPos,
    ChUnit,
    ChUnitMul,
    ChDacqName,
    SssFrame,
    SssJob,
    SssOrigin,
    SssOrdIn,
    SssOrdOut,
    SssNmag,
    SssComponents,
    SssCalChans,
    SssCalCorrs,
    SssStCorr,
    SssBaseIn,
    SssBaseOut,
    SssBaseVirt,
    SssNorm,
    SssIterate,
    SssNfree,
    SssStLength,
    GantryType,
    GantryModel,
    GantryAngle,
    SssOperator,
    SssPsinv,
    SssCtc,
    DataBuffer,
    DataSkip,
    Epoch,
    DataSkipSamp,
    TimeStamp,
    ChCoilType,
    ChCoordFrame,
    SubjId,
    SubjFirstName,
    SubjMiddleName,
    SubjLastName,
    SubjBirthDay,
    SubjSex,
    SubjHand,
    SubjWeight,
    SubjHeight,
    SubjComment,
    SubjHisId,
    ProjId,
    ProjName,
    ProjAim,
    ProjPersons,
    ProjComment,
    EventChannels,
    EventList,
    EventChannel,
    EventBits,
    SquidBias,
    SquidOffset,
    SquidGate,
    DecouplerMatrix,
    CtmOpenAmps,
    CtmOpenPhase,
    CtmClosAmps,
    CtmClosPhase,
    CtmClosDote,
    CtmOpenDote,
    CtmExciFreq,
    RefPath,
    VolumeType,
    MriSourceFormat,
    MriPixelEncoding,
    MriPixelDataOffset,
    MriPixelScale,
    MriPixelData,
    MriPixelOverlayEncoding,
    MriPixelOverlayData,
    MriBoundingBox,
    MriWidth,
    MriWidthM,
    MriHeight,
    MriHeightM,
    MriDepth,
    MriDepthM,
    MriThickness,
    MriSceneAim,
    MriCalibrationScale,
    MriCalibrationOffset,
    MriOrigSourcePath,
    MriOrigSourceFormat,
    MriOrigPixelEncoding,
    MriOrigPixelDataOffset,
    MriTime,
    MriVoxelData,
    MriVoxelEncoding,
    VoxelNchannels,
    MriDiffusionWeight,
    MriDiffusionParam,
    MriMrilabSetup,
    MriSegRegionId,
    ConductorModelKind,
    SphereOrigin,
    SphereCoordFrame,
    SphereLayers,
    BemSurfId,
    BemSurfName,
    BemSurfNnode,
    BemSurfNtri,
    BemSurfNodes,
    BemSurfTriangles,
    BemSurfNormals,
    BemSurfCurvs,
    BemSurfCurvValues,
    BemPotSolution,
    BemApprox,
    BemCoordFrame,
    BemSigma,
    SourceDipole,
    BeamformerInstructions,
    XfitLeadProducts,
    XfitMapProducts,
    XfitGradMapProducts,
    XfitVolIntegration,
    XfitIntegrationRadius,
    XfitConductorModelName,
    XfitConductorModelTransName,
    XfitContSurfType,
    ProjItemKind,
    ProjItemTime,
    ProjItemIgnChs,
    ProjItemNvec,
    ProjItemVectors,
    ProjItemDefinition,
    ProjItemChNameList,
    XplotterLayout,
    VolId,
    VolName,
    VolOwnerId,
    VolOwnerName,
    VolOwnerRealName,
    VolType,
    VolHost,
    VolRealRoot,
    VolSymbolicRoot,
    VolMountPoint,
    VolBlocks,
    VolFreeBlocks,
    VolAvailBlocks,
    VolBlockSize,
    VolDirectory,
    MemDataBuffer,
    Code(i32),
}

impl Kind {
    pub fn from_code(code: i32) -> Self {
        match code {
            100 => Kind::FileId,
            101 => Kind::DirPointer,
            102 => Kind::Dir,
            103 => Kind::BlockId,
            104 => Kind::BlockStart,
            105 => Kind::BlockEnd,
            106 => Kind::FreeList,
            107 => Kind::FreeBlock,
            108 => Kind::Nop,
            109 => Kind::ParentFileId,
            110 => Kind::ParentBlockId,
            111 => Kind::BlockName,
            112 => Kind::BlockVersion,
            113 => Kind::Creator,
            114 => Kind::Modifier,
            115 => Kind::RefRole,
            116 => Kind::RefFileId,
            117 => Kind::RefFileNum,
            118 => Kind::RefFileName,
            120 => Kind::RefBlockId,
            150 => Kind::DacqPars,
            151 => Kind::DacqStim,
            152 => Kind::DeviceType,
            153 => Kind::DeviceModel,
            154 => Kind::DeviceSerial,
            155 => Kind::DeviceSite,
            156 => Kind::HeLevelRaw,
            157 => Kind::HeliumLevel,
            158 => Kind::OrigFileGuid,
            159 => Kind::UtcOffset,
            200 => Kind::Nchan,
            201 => Kind::Sfreq,
            202 => Kind::DataPack,
            203 => Kind::ChInfo,
            204 => Kind::MeasDate,
            205 => Kind::Subject,
            206 => Kind::Description,
            207 => Kind::Nave,
            208 => Kind::FirstSample,
            209 => Kind::LastSample,
            210 => Kind::AspectKind,
            211 => Kind::RefEvent,
            212 => Kind::Experimenter,
            213 => Kind::DigPoint,
            214 => Kind::ChPosVec,
            215 => Kind::HpiSlopes,
            216 => Kind::HpiNcoil,
            217 => Kind::ReqEvent,
            218 => Kind::ReqLimit,
            219 => Kind::Lowpass,
            220 => Kind::BadChs,
            221 => Kind::ArtefRemoval,
            222 => Kind::CoordTrans,
            223 => Kind::Highpass,
            224 => Kind::ChCalsVec,
            225 => Kind::HpiBadChs,
            226 => Kind::HpiCorrCoeff,
            227 => Kind::EventComment,
            228 => Kind::NoSamples,
            229 => Kind::FirstTime,
            230 => Kind::SubaveSize,
            231 => Kind::SubaveFirst,
            233 => Kind::Name,
            234 => Kind::DigString,
            235 => Kind::LineFreq,
            236 => Kind::HpiCoilFreq,
            237 => Kind::SignalChannel,
            240 => Kind::HpiCoilMoments,
            241 => Kind::HpiFitGoodness,
            242 => Kind::HpiFitAccept,
            243 => Kind::HpiFitGoodLimit,
            244 => Kind::HpiFitDistLimit,
            245 => Kind::HpiCoilNo,
            246 => Kind::HpiCoilsUsed,
            247 => Kind::HpiDigitizationOrder,
            250 => Kind::ChScanNo,
            251 => Kind::ChLogicalNo,
            252 => Kind::ChKind,
            253 => Kind::ChRange,
            254 => Kind::ChCal,
            255 => Kind::ChPos,
            256 => Kind::ChUnit,
            257 => Kind::ChUnitMul,
            258 => Kind::ChDacqName,
            263 => Kind::SssFrame,
            264 => Kind::SssJob,
            265 => Kind::SssOrigin,
            266 => Kind::SssOrdIn,
            267 => Kind::SssOrdOut,
            268 => Kind::SssNmag,
            269 => Kind::SssComponents,
            270 => Kind::SssCalChans,
            271 => Kind::SssCalCorrs,
            272 => Kind::SssStCorr,
            273 => Kind::SssBaseIn,
            274 => Kind::SssBaseOut,
            275 => Kind::SssBaseVirt,
            276 => Kind::SssNorm,
            277 => Kind::SssIterate,
            278 => Kind::SssNfree,
            279 => Kind::SssStLength,
            280 => Kind::GantryType,
            281 => Kind::GantryModel,
            282 => Kind::GantryAngle,
            290 => Kind::SssOperator,
            291 => Kind::SssPsinv,
            292 => Kind::SssCtc,
            300 => Kind::DataBuffer,
            301 => Kind::DataSkip,
            302 => Kind::Epoch,
            303 => Kind::DataSkipSamp,
            305 => Kind::TimeStamp,
            350 => Kind::ChCoilType,
            351 => Kind::ChCoordFrame,
            400 => Kind::SubjId,
            401 => Kind::SubjFirstName,
            402 => Kind::SubjMiddleName,
            403 => Kind::SubjLastName,
            404 => Kind::SubjBirthDay,
            405 => Kind::SubjSex,
            406 => Kind::SubjHand,
            407 => Kind::SubjWeight,
            408 => Kind::SubjHeight,
            409 => Kind::SubjComment,
            410 => Kind::SubjHisId,
            500 => Kind::ProjId,
            501 => Kind::ProjName,
            502 => Kind::ProjAim,
            503 => Kind::ProjPersons,
            504 => Kind::ProjComment,
            600 => Kind::EventChannels,
            601 => Kind::EventList,
            602 => Kind::EventChannel,
            603 => Kind::EventBits,
            701 => Kind::SquidBias,
            702 => Kind::SquidOffset,
            703 => Kind::SquidGate,
            800 => Kind::DecouplerMatrix,
            801 => Kind::CtmOpenAmps,
            802 => Kind::CtmOpenPhase,
            803 => Kind::CtmClosAmps,
            804 => Kind::CtmClosPhase,
            805 => Kind::CtmClosDote,
            806 => Kind::CtmOpenDote,
            807 => Kind::CtmExciFreq,
            1101 => Kind::RefPath,
            2001 => Kind::VolumeType,
            2002 => Kind::MriSourceFormat,
            2003 => Kind::MriPixelEncoding,
            2004 => Kind::MriPixelDataOffset,
            2005 => Kind::MriPixelScale,
            2006 => Kind::MriPixelData,
            2007 => Kind::MriPixelOverlayEncoding,
            2008 => Kind::MriPixelOverlayData,
            2009 => Kind::MriBoundingBox,
            2010 => Kind::MriWidth,
            2011 => Kind::MriWidthM,
            2012 => Kind::MriHeight,
            2013 => Kind::MriHeightM,
            2014 => Kind::MriDepth,
            2015 => Kind::MriDepthM,
            2016 => Kind::MriThickness,
            2017 => Kind::MriSceneAim,
            2018 => Kind::MriCalibrationScale,
            2019 => Kind::MriCalibrationOffset,
            2020 => Kind::MriOrigSourcePath,
            2021 => Kind::MriOrigSourceFormat,
            2022 => Kind::MriOrigPixelEncoding,
            2023 => Kind::MriOrigPixelDataOffset,
            2024 => Kind::MriTime,
            2030 => Kind::MriVoxelData,
            2031 => Kind::MriVoxelEncoding,
            2032 => Kind::VoxelNchannels,
            2040 => Kind::MriDiffusionWeight,
            2041 => Kind::MriDiffusionParam,
            2100 => Kind::MriMrilabSetup,
            2200 => Kind::MriSegRegionId,
            3000 => Kind::ConductorModelKind,
            3001 => Kind::SphereOrigin,
            3002 => Kind::SphereCoordFrame,
            3003 => Kind::SphereLayers,
            3101 => Kind::BemSurfId,
            3102 => Kind::BemSurfName,
            3103 => Kind::BemSurfNnode,
            3104 => Kind::BemSurfNtri,
            3105 => Kind::BemSurfNodes,
            3106 => Kind::BemSurfTriangles,
            3107 => Kind::BemSurfNormals,
            3108 => Kind::BemSurfCurvs,
            3109 => Kind::BemSurfCurvValues,
            3110 => Kind::BemPotSolution,
            3111 => Kind::BemApprox,
            3112 => Kind::BemCoordFrame,
            3113 => Kind::BemSigma,
            3201 => Kind::SourceDipole,
            3300 => Kind::BeamformerInstructions,
            3401 => Kind::XfitLeadProducts,
            3402 => Kind::XfitMapProducts,
            3403 => Kind::XfitGradMapProducts,
            3404 => Kind::XfitVolIntegration,
            3405 => Kind::XfitIntegrationRadius,
            3406 => Kind::XfitConductorModelName,
            3407 => Kind::XfitConductorModelTransName,
            3408 => Kind::XfitContSurfType,
            3411 => Kind::ProjItemKind,
            3412 => Kind::ProjItemTime,
            3413 => Kind::ProjItemIgnChs,
            3414 => Kind::ProjItemNvec,
            3415 => Kind::ProjItemVectors,
            3416 => Kind::ProjItemDefinition,
            3417 => Kind::ProjItemChNameList,
            3501 => Kind::XplotterLayout,
            4001 => Kind::VolId,
            4002 => Kind::VolName,
            4003 => Kind::VolOwnerId,
            4004 => Kind::VolOwnerName,
            4005 => Kind::VolOwnerRealName,
            4006 => Kind::VolType,
            4007 => Kind::VolHost,
            4008 => Kind::VolRealRoot,
            4009 => Kind::VolSymbolicRoot,
            4010 => Kind::VolMountPoint,
            4011 => Kind::VolBlocks,
            4012 => Kind::VolFreeBlocks,
            4013 => Kind::VolAvailBlocks,
            4014 => Kind::VolBlockSize,
            4015 => Kind::VolDirectory,
            10300 => Kind::MemDataBuffer,
            _ => Kind::Code(code),
        }
    }
}

pub enum Block {
    Root,
    Meas,
    MeasInfo,
    RawData,
    ProcessedData,
    Evoked,
    Aspect,
    Subject,
    Isotrak,
    HpiMeas,
    HpiResult,
    HpiCoil,
    Project,
    ContinuousData,
    ChInfo,
    Void,
    Events,
    Index,
    DacqPars,
    Ref,
    IasRawData,
    IasAspect,
    HpiSubsystem,
    PhantomSubsystem,
    StatusSubsystem,
    DeviceInfo,
    HeliumInfo,
    ChannelInfo,
    StructuralData,
    VolumeData,
    VolumeSlice,
    Scenery,
    Scene,
    MriSeg,
    MriSegRegion,
    Sphere,
    Bem,
    BemSurf,
    ConductorModel,
    Ssp,
    SspItem,
    XfitAux,
    FiffCov,
    BadChannels,
    VolInfo,
    DataCorrection,
    ChannelsDecoupler,
    SssInfo,
    SssCalAdjust,
    SssStInfo,
    SssBases,
    SssOperator,
    CtMeas,
    SssExpansion,
    Ias,
    ProcessingHistory,
    ProcessingRecord,
    Code(i32),
}

impl Block {
    pub fn from_code(code: i32) -> Self {
        match code {
            999 => Block::Root,
            100 => Block::Meas,
            101 => Block::MeasInfo,
            102 => Block::RawData,
            103 => Block::ProcessedData,
            104 => Block::Evoked,
            105 => Block::Aspect,
            106 => Block::Subject,
            107 => Block::Isotrak,
            108 => Block::HpiMeas,
            109 => Block::HpiResult,
            110 => Block::HpiCoil,
            111 => Block::Project,
            112 => Block::ContinuousData,
            113 => Block::ChInfo,
            114 => Block::Void,
            115 => Block::Events,
            116 => Block::Index,
            117 => Block::DacqPars,
            118 => Block::Ref,
            119 => Block::IasRawData,
            120 => Block::IasAspect,
            121 => Block::HpiSubsystem,
            122 => Block::PhantomSubsystem,
            123 => Block::StatusSubsystem,
            124 => Block::DeviceInfo,
            125 => Block::HeliumInfo,
            126 => Block::ChannelInfo,
            200 => Block::StructuralData,
            201 => Block::VolumeData,
            202 => Block::VolumeSlice,
            203 => Block::Scenery,
            204 => Block::Scene,
            205 => Block::MriSeg,
            206 => Block::MriSegRegion,
            300 => Block::Sphere,
            310 => Block::Bem,
            311 => Block::BemSurf,
            312 => Block::ConductorModel,
            313 => Block::Ssp,
            314 => Block::SspItem,
            315 => Block::XfitAux,
            355 => Block::FiffCov,
            359 => Block::BadChannels,
            400 => Block::VolInfo,
            500 => Block::DataCorrection,
            501 => Block::ChannelsDecoupler,
            502 => Block::SssInfo,
            503 => Block::SssCalAdjust,
            504 => Block::SssStInfo,
            505 => Block::SssBases,
            506 => Block::SssOperator,
            507 => Block::CtMeas,
            508 => Block::SssExpansion,
            510 => Block::Ias,
            900 => Block::ProcessingHistory,
            901 => Block::ProcessingRecord,
            _ => Block::Code(code),
        }
    }
}