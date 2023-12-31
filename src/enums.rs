//! FIFF tag enums
use anyhow::Result;
use serde::Serialize;

// tag code, see fiff/tags.tsv
// is currently missing the MNE-specific tags.
// carries a catchall Code(i32) variant to hold any codes not recognised here
#[derive(Debug, PartialEq, Default, Clone, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DataTagKind {
    #[default]
    FileId,
    DirPointer,
    Dir,
    // BlockId,
    // BlockStart,
    // BlockEnd,
    FreeList,
    FreeBlock,
    Nop,
    ParentFileId,
    // ParentBlockId,
    // BlockName,
    // BlockVersion,
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
}

impl DataTagKind {
    pub fn from_code(code: i32) -> Result<Self> {
        let out = match code {
            100 => DataTagKind::FileId,
            101 => DataTagKind::DirPointer,
            102 => DataTagKind::Dir,
            106 => DataTagKind::FreeList,
            107 => DataTagKind::FreeBlock,
            108 => DataTagKind::Nop,
            109 => DataTagKind::ParentFileId,
            113 => DataTagKind::Creator,
            114 => DataTagKind::Modifier,
            115 => DataTagKind::RefRole,
            116 => DataTagKind::RefFileId,
            117 => DataTagKind::RefFileNum,
            118 => DataTagKind::RefFileName,
            120 => DataTagKind::RefBlockId,
            150 => DataTagKind::DacqPars,
            151 => DataTagKind::DacqStim,
            152 => DataTagKind::DeviceType,
            153 => DataTagKind::DeviceModel,
            154 => DataTagKind::DeviceSerial,
            155 => DataTagKind::DeviceSite,
            156 => DataTagKind::HeLevelRaw,
            157 => DataTagKind::HeliumLevel,
            158 => DataTagKind::OrigFileGuid,
            159 => DataTagKind::UtcOffset,
            200 => DataTagKind::Nchan,
            201 => DataTagKind::Sfreq,
            202 => DataTagKind::DataPack,
            203 => DataTagKind::ChInfo,
            204 => DataTagKind::MeasDate,
            205 => DataTagKind::Subject,
            206 => DataTagKind::Description,
            207 => DataTagKind::Nave,
            208 => DataTagKind::FirstSample,
            209 => DataTagKind::LastSample,
            210 => DataTagKind::AspectKind,
            211 => DataTagKind::RefEvent,
            212 => DataTagKind::Experimenter,
            213 => DataTagKind::DigPoint,
            214 => DataTagKind::ChPosVec,
            215 => DataTagKind::HpiSlopes,
            216 => DataTagKind::HpiNcoil,
            217 => DataTagKind::ReqEvent,
            218 => DataTagKind::ReqLimit,
            219 => DataTagKind::Lowpass,
            220 => DataTagKind::BadChs,
            221 => DataTagKind::ArtefRemoval,
            222 => DataTagKind::CoordTrans,
            223 => DataTagKind::Highpass,
            224 => DataTagKind::ChCalsVec,
            225 => DataTagKind::HpiBadChs,
            226 => DataTagKind::HpiCorrCoeff,
            227 => DataTagKind::EventComment,
            228 => DataTagKind::NoSamples,
            229 => DataTagKind::FirstTime,
            230 => DataTagKind::SubaveSize,
            231 => DataTagKind::SubaveFirst,
            233 => DataTagKind::Name,
            234 => DataTagKind::DigString,
            235 => DataTagKind::LineFreq,
            236 => DataTagKind::HpiCoilFreq,
            237 => DataTagKind::SignalChannel,
            240 => DataTagKind::HpiCoilMoments,
            241 => DataTagKind::HpiFitGoodness,
            242 => DataTagKind::HpiFitAccept,
            243 => DataTagKind::HpiFitGoodLimit,
            244 => DataTagKind::HpiFitDistLimit,
            245 => DataTagKind::HpiCoilNo,
            246 => DataTagKind::HpiCoilsUsed,
            247 => DataTagKind::HpiDigitizationOrder,
            250 => DataTagKind::ChScanNo,
            251 => DataTagKind::ChLogicalNo,
            252 => DataTagKind::ChKind,
            253 => DataTagKind::ChRange,
            254 => DataTagKind::ChCal,
            255 => DataTagKind::ChPos,
            256 => DataTagKind::ChUnit,
            257 => DataTagKind::ChUnitMul,
            258 => DataTagKind::ChDacqName,
            263 => DataTagKind::SssFrame,
            264 => DataTagKind::SssJob,
            265 => DataTagKind::SssOrigin,
            266 => DataTagKind::SssOrdIn,
            267 => DataTagKind::SssOrdOut,
            268 => DataTagKind::SssNmag,
            269 => DataTagKind::SssComponents,
            270 => DataTagKind::SssCalChans,
            271 => DataTagKind::SssCalCorrs,
            272 => DataTagKind::SssStCorr,
            273 => DataTagKind::SssBaseIn,
            274 => DataTagKind::SssBaseOut,
            275 => DataTagKind::SssBaseVirt,
            276 => DataTagKind::SssNorm,
            277 => DataTagKind::SssIterate,
            278 => DataTagKind::SssNfree,
            279 => DataTagKind::SssStLength,
            280 => DataTagKind::GantryType,
            281 => DataTagKind::GantryModel,
            282 => DataTagKind::GantryAngle,
            290 => DataTagKind::SssOperator,
            291 => DataTagKind::SssPsinv,
            292 => DataTagKind::SssCtc,
            300 => DataTagKind::DataBuffer,
            301 => DataTagKind::DataSkip,
            302 => DataTagKind::Epoch,
            303 => DataTagKind::DataSkipSamp,
            305 => DataTagKind::TimeStamp,
            350 => DataTagKind::ChCoilType,
            351 => DataTagKind::ChCoordFrame,
            400 => DataTagKind::SubjId,
            401 => DataTagKind::SubjFirstName,
            402 => DataTagKind::SubjMiddleName,
            403 => DataTagKind::SubjLastName,
            404 => DataTagKind::SubjBirthDay,
            405 => DataTagKind::SubjSex,
            406 => DataTagKind::SubjHand,
            407 => DataTagKind::SubjWeight,
            408 => DataTagKind::SubjHeight,
            409 => DataTagKind::SubjComment,
            410 => DataTagKind::SubjHisId,
            500 => DataTagKind::ProjId,
            501 => DataTagKind::ProjName,
            502 => DataTagKind::ProjAim,
            503 => DataTagKind::ProjPersons,
            504 => DataTagKind::ProjComment,
            600 => DataTagKind::EventChannels,
            601 => DataTagKind::EventList,
            602 => DataTagKind::EventChannel,
            603 => DataTagKind::EventBits,
            701 => DataTagKind::SquidBias,
            702 => DataTagKind::SquidOffset,
            703 => DataTagKind::SquidGate,
            800 => DataTagKind::DecouplerMatrix,
            801 => DataTagKind::CtmOpenAmps,
            802 => DataTagKind::CtmOpenPhase,
            803 => DataTagKind::CtmClosAmps,
            804 => DataTagKind::CtmClosPhase,
            805 => DataTagKind::CtmClosDote,
            806 => DataTagKind::CtmOpenDote,
            807 => DataTagKind::CtmExciFreq,
            1101 => DataTagKind::RefPath,
            2001 => DataTagKind::VolumeType,
            2002 => DataTagKind::MriSourceFormat,
            2003 => DataTagKind::MriPixelEncoding,
            2004 => DataTagKind::MriPixelDataOffset,
            2005 => DataTagKind::MriPixelScale,
            2006 => DataTagKind::MriPixelData,
            2007 => DataTagKind::MriPixelOverlayEncoding,
            2008 => DataTagKind::MriPixelOverlayData,
            2009 => DataTagKind::MriBoundingBox,
            2010 => DataTagKind::MriWidth,
            2011 => DataTagKind::MriWidthM,
            2012 => DataTagKind::MriHeight,
            2013 => DataTagKind::MriHeightM,
            2014 => DataTagKind::MriDepth,
            2015 => DataTagKind::MriDepthM,
            2016 => DataTagKind::MriThickness,
            2017 => DataTagKind::MriSceneAim,
            2018 => DataTagKind::MriCalibrationScale,
            2019 => DataTagKind::MriCalibrationOffset,
            2020 => DataTagKind::MriOrigSourcePath,
            2021 => DataTagKind::MriOrigSourceFormat,
            2022 => DataTagKind::MriOrigPixelEncoding,
            2023 => DataTagKind::MriOrigPixelDataOffset,
            2024 => DataTagKind::MriTime,
            2030 => DataTagKind::MriVoxelData,
            2031 => DataTagKind::MriVoxelEncoding,
            2032 => DataTagKind::VoxelNchannels,
            2040 => DataTagKind::MriDiffusionWeight,
            2041 => DataTagKind::MriDiffusionParam,
            2100 => DataTagKind::MriMrilabSetup,
            2200 => DataTagKind::MriSegRegionId,
            3000 => DataTagKind::ConductorModelKind,
            3001 => DataTagKind::SphereOrigin,
            3002 => DataTagKind::SphereCoordFrame,
            3003 => DataTagKind::SphereLayers,
            3101 => DataTagKind::BemSurfId,
            3102 => DataTagKind::BemSurfName,
            3103 => DataTagKind::BemSurfNnode,
            3104 => DataTagKind::BemSurfNtri,
            3105 => DataTagKind::BemSurfNodes,
            3106 => DataTagKind::BemSurfTriangles,
            3107 => DataTagKind::BemSurfNormals,
            3108 => DataTagKind::BemSurfCurvs,
            3109 => DataTagKind::BemSurfCurvValues,
            3110 => DataTagKind::BemPotSolution,
            3111 => DataTagKind::BemApprox,
            3112 => DataTagKind::BemCoordFrame,
            3113 => DataTagKind::BemSigma,
            3201 => DataTagKind::SourceDipole,
            3300 => DataTagKind::BeamformerInstructions,
            3401 => DataTagKind::XfitLeadProducts,
            3402 => DataTagKind::XfitMapProducts,
            3403 => DataTagKind::XfitGradMapProducts,
            3404 => DataTagKind::XfitVolIntegration,
            3405 => DataTagKind::XfitIntegrationRadius,
            3406 => DataTagKind::XfitConductorModelName,
            3407 => DataTagKind::XfitConductorModelTransName,
            3408 => DataTagKind::XfitContSurfType,
            3411 => DataTagKind::ProjItemKind,
            3412 => DataTagKind::ProjItemTime,
            3413 => DataTagKind::ProjItemIgnChs,
            3414 => DataTagKind::ProjItemNvec,
            3415 => DataTagKind::ProjItemVectors,
            3416 => DataTagKind::ProjItemDefinition,
            3417 => DataTagKind::ProjItemChNameList,
            3501 => DataTagKind::XplotterLayout,
            4001 => DataTagKind::VolId,
            4002 => DataTagKind::VolName,
            4003 => DataTagKind::VolOwnerId,
            4004 => DataTagKind::VolOwnerName,
            4005 => DataTagKind::VolOwnerRealName,
            4006 => DataTagKind::VolType,
            4007 => DataTagKind::VolHost,
            4008 => DataTagKind::VolRealRoot,
            4009 => DataTagKind::VolSymbolicRoot,
            4010 => DataTagKind::VolMountPoint,
            4011 => DataTagKind::VolBlocks,
            4012 => DataTagKind::VolFreeBlocks,
            4013 => DataTagKind::VolAvailBlocks,
            4014 => DataTagKind::VolBlockSize,
            4015 => DataTagKind::VolDirectory,
            10300 => DataTagKind::MemDataBuffer,
            _ => {
                return Err(anyhow::anyhow!("Unrecognized data tag code: {code}"));
            }
        };

        Ok(out)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockTagKind {
    BlockId,
    BlockStart,
    BlockEnd,
    ParentBlockId,
    BlockName,
    BlockVersion,
}

impl BlockTagKind {
    pub fn from_code(code: i32) -> Result<Self, String> {
        match code {
            103 => Ok(BlockTagKind::BlockId),
            104 => Ok(BlockTagKind::BlockStart),
            105 => Ok(BlockTagKind::BlockEnd),
            110 => Ok(BlockTagKind::ParentBlockId),
            111 => Ok(BlockTagKind::BlockName),
            112 => Ok(BlockTagKind::BlockVersion),
            _ => Err(format!("could not convert code {} to block", code)),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub enum BlockKind {
    #[default]
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

impl BlockKind {
    pub fn from_code(code: i32) -> Self {
        match code {
            999 => BlockKind::Root,
            100 => BlockKind::Meas,
            101 => BlockKind::MeasInfo,
            102 => BlockKind::RawData,
            103 => BlockKind::ProcessedData,
            104 => BlockKind::Evoked,
            105 => BlockKind::Aspect,
            106 => BlockKind::Subject,
            107 => BlockKind::Isotrak,
            108 => BlockKind::HpiMeas,
            109 => BlockKind::HpiResult,
            110 => BlockKind::HpiCoil,
            111 => BlockKind::Project,
            112 => BlockKind::ContinuousData,
            113 => BlockKind::ChInfo,
            114 => BlockKind::Void,
            115 => BlockKind::Events,
            116 => BlockKind::Index,
            117 => BlockKind::DacqPars,
            118 => BlockKind::Ref,
            119 => BlockKind::IasRawData,
            120 => BlockKind::IasAspect,
            121 => BlockKind::HpiSubsystem,
            122 => BlockKind::PhantomSubsystem,
            123 => BlockKind::StatusSubsystem,
            124 => BlockKind::DeviceInfo,
            125 => BlockKind::HeliumInfo,
            126 => BlockKind::ChannelInfo,
            200 => BlockKind::StructuralData,
            201 => BlockKind::VolumeData,
            202 => BlockKind::VolumeSlice,
            203 => BlockKind::Scenery,
            204 => BlockKind::Scene,
            205 => BlockKind::MriSeg,
            206 => BlockKind::MriSegRegion,
            300 => BlockKind::Sphere,
            310 => BlockKind::Bem,
            311 => BlockKind::BemSurf,
            312 => BlockKind::ConductorModel,
            313 => BlockKind::Ssp,
            314 => BlockKind::SspItem,
            315 => BlockKind::XfitAux,
            355 => BlockKind::FiffCov,
            359 => BlockKind::BadChannels,
            400 => BlockKind::VolInfo,
            500 => BlockKind::DataCorrection,
            501 => BlockKind::ChannelsDecoupler,
            502 => BlockKind::SssInfo,
            503 => BlockKind::SssCalAdjust,
            504 => BlockKind::SssStInfo,
            505 => BlockKind::SssBases,
            506 => BlockKind::SssOperator,
            507 => BlockKind::CtMeas,
            508 => BlockKind::SssExpansion,
            510 => BlockKind::Ias,
            900 => BlockKind::ProcessingHistory,
            901 => BlockKind::ProcessingRecord,
            _ => BlockKind::Code(code),
        }
    }
}
