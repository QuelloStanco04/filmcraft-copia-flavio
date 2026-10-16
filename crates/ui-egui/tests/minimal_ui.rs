//! Headless UI tests of the minimal interface (`FilmcraftApp::with_minimal_ui`, `?minimal` on the
//! web): a header with only Quick Export, no Discord or Tools, monitor buttons that have shortcuts
//! hidden, a starting layout (Program + meters over Timeline + a narrow column) whose panels stay
//! free, and a timeline with one chip per track. Off by default, where nothing changes.

use egui::{Event, Modifiers, PointerButton, pos2};
use egui_kittest::Harness;
use filmcraft_engine::Session;
use filmcraft_ui_egui::FilmcraftApp;
use filmcraft_ui_egui::dock::PanelKind;
use serde_json::json;

fn harness(minimal: bool) -> Harness<'static, FilmcraftApp> {
    let mut s = Session::default();
    s.execute("file.openDemoProject", json!({})).unwrap();
    let app = FilmcraftApp::new(s).with_minimal_ui(minimal);
    let mut harness = Harness::builder().with_size(egui::vec2(1600.0, 980.0)).with_max_steps(10_000).build_eframe(move |_cc| app);
    for _ in 0..6 {
        harness.step();
    }
    harness
}

fn has(h: &Harness<'static, FilmcraftApp>, prefix: &str) -> bool {
    !h.state().auto.query(prefix).is_empty()
}

#[test]
fn the_default_interface_is_unchanged() {
    let h = harness(false);
    assert!(has(&h, "header.discord"));
    assert!(has(&h, "tools."), "the Tools panel is shown");
    assert!(has(&h, "program.transport.markers.add"));
    assert!(has(&h, "program.transport.sequence.lift"));
    assert!(has(&h, "program.transport.editor"));
    assert!(has(&h, "timeline.track.V1.sourcePatch"));
    assert!(has(&h, "timeline.track.V1.syncLock"));
    assert!(h.state().ui.dock.contains(PanelKind::History));
    assert!(h.state().ui.dock.contains(PanelKind::EssentialGraphics));
    assert!(h.state().ui.dock.contains(PanelKind::Text));
    assert!(h.state().ui_error.is_none(), "{:?}", h.state().ui_error);
}

#[test]
fn the_minimal_interface_drops_the_buttons_that_have_shortcuts() {
    let h = harness(true);
    assert!(!has(&h, "header.discord"), "Discord button hidden");
    assert!(!has(&h, "tools."), "Tools panel hidden");
    assert!(!has(&h, "program.transport.markers.add"));
    assert!(!has(&h, "program.transport.markers.markIn"));
    assert!(!has(&h, "program.transport.sequence.lift"));
    assert!(!has(&h, "program.transport.sequence.extract"));
    assert!(!has(&h, "program.transport.exportFrame"));
    assert!(!has(&h, "program.transport.editor"), "no button editor");
    assert!(has(&h, "program.transport.media.toggleProxies"), "the proxy toggle stays");
    assert!(has(&h, "program.transport.playback.toggle"), "play stays");
    assert!(has(&h, "program.transport.playhead.stepBack"));
    assert!(has(&h, "program.transport.playhead.stepForward"));
    assert!(h.state().ui_error.is_none(), "{:?}", h.state().ui_error);
}

#[test]
fn the_minimal_header_keeps_only_quick_export() {
    let h = harness(true);
    assert!(has(&h, "header.quickExport"));
    for id in ["fullscreen", "volume", "search", "notifications", "appearance", "workspaces", "workspaceName"] {
        assert!(!has(&h, &format!("header.{id}")), "header.{id} should be hidden");
    }
    assert!(!has(&h, "header.home") && !has(&h, "header.mode.edit"), "no Home / mode tabs");
    let d = harness(false);
    assert!(has(&d, "header.home") && has(&d, "header.mode.edit"), "Home / mode tabs by default");
    for id in ["quickExport", "fullscreen", "volume", "search", "notifications", "appearance", "workspaces"] {
        assert!(has(&d, &format!("header.{id}")), "header.{id} missing by default");
    }
}

#[test]
fn the_minimal_layout_is_a_starting_point_with_the_agreed_panels() {
    let h = harness(true);
    for panel in [PanelKind::History, PanelKind::EssentialGraphics, PanelKind::Tools, PanelKind::Project, PanelKind::Source] {
        assert!(!h.state().ui.dock.contains(panel), "{panel:?} still docked");
    }
    for panel in [PanelKind::Program, PanelKind::AudioMeters, PanelKind::Timeline, PanelKind::Properties, PanelKind::EffectControls, PanelKind::Text] {
        assert!(h.state().ui.dock.contains(panel), "{panel:?} missing");
    }
    assert!(h.state().ui_error.is_none(), "{:?}", h.state().ui_error);
}

#[test]
fn panels_stay_free_in_the_minimal_interface() {
    let mut h = harness(true);
    // a panel closed at launch can be opened again, and stays
    h.state_mut().show_panel(PanelKind::Project);
    h.state_mut().show_panel(PanelKind::Tools);
    for _ in 0..4 {
        h.step();
    }
    assert!(h.state().ui.dock.contains(PanelKind::Project), "Project can be reopened");
    assert!(h.state().ui.dock.contains(PanelKind::Tools), "Tools can be reopened and is not closed again");
    // and a panel can be closed
    h.state_mut().ui.dock.close(PanelKind::Text);
    for _ in 0..4 {
        h.step();
    }
    assert!(!h.state().ui.dock.contains(PanelKind::Text));
}

#[test]
fn the_minimal_timeline_has_a_thin_video_lane_tall_audio_lanes_and_one_chip_per_track() {
    let h = harness(true);
    for gone in ["sourcePatch", "target", "syncLock", "locked", "solo"] {
        assert!(!has(&h, &format!("timeline.track.V1.{gone}")) && !has(&h, &format!("timeline.track.A1.{gone}")), "{gone} should be gone");
    }
    assert!(has(&h, "timeline.track.V1.enabled"), "video chip");
    assert!(has(&h, "timeline.track.A1.muted"), "audio chip");
    assert!(!has(&h, "timeline.toggle.marker"), "no marker button");
    assert!(has(&h, "timeline.toggle.snap"));
    assert!(!has(&h, "timeline.toggle.captions"), "the caption button did nothing and is gone");
    assert!(has(&h, "timeline.toggle.nest") && has(&h, "timeline.toggle.linked") && has(&h, "timeline.toggle.settings"));
    assert!(!has(&h, "program.scrubBar"), "no scrub bar under the Program monitor");
    assert!(has(&h, "program.transport.playback.toggle"));
    let tl = &h.state().ui.timeline;
    assert!(tl.header_w < 60.0, "narrow header column: {}", tl.header_w);
    assert!(tl.video_track_h < 40.0, "thin video lane: {}", tl.video_track_h);
    assert!(tl.split < 0.4, "most of the height is for the audio lanes: {}", tl.split);
    assert!(tl.audio_track_h > 50.0, "tall audio lanes for the waveform: {}", tl.audio_track_h);
}

#[test]
fn clicking_the_chip_mutes_an_audio_track_and_hides_a_video_track() {
    let mut h = harness(true);
    let seq = |h: &Harness<'static, FilmcraftApp>| h.state().session.active_sequence().map(|s| (s.video_tracks[0].enabled, s.audio_tracks[0].muted));
    let before = seq(&h).unwrap();
    for id in ["timeline.track.V1.enabled", "timeline.track.A1.muted"] {
        let r = h.state().auto.query(id).first().map(|e| e.rect).unwrap();
        let pos = pos2(r[0] + r[2] / 2.0, r[1] + r[3] / 2.0);
        h.event(Event::PointerMoved(pos));
        h.step();
        for pressed in [true, false] {
            h.event(Event::PointerButton { pos, button: PointerButton::Primary, pressed, modifiers: Modifiers::NONE });
            h.step();
        }
        h.run_steps(3);
    }
    let after = seq(&h).unwrap();
    assert_eq!(after.0, !before.0, "video track toggled");
    assert_eq!(after.1, !before.1, "audio track toggled");
}

#[test]
fn the_minimal_interface_wastes_no_rows() {
    let h = harness(true);
    assert!(!h.state().ui.show_status_bar, "no hint line under the panels");
    assert!(h.state().tokens.tab_h < 30.0, "slim panel tab bars: {}", h.state().tokens.tab_h);
    // the Program monitor's timecode and its transport buttons share one row
    let y = |id: &str| h.state().auto.query(id).first().map(|e| e.rect[1] + e.rect[3] / 2.0).unwrap_or(f32::NAN);
    let (tc, play) = (y("program.timecode"), y("program.transport.playback.toggle"));
    assert!((tc - play).abs() < 8.0, "timecode row {tc} vs transport row {play}");
    assert!(h.state().tokens.gap <= 3.0, "thin dividers: {}", h.state().tokens.gap);
    assert!(h.state().tokens.hide_timeline_strip, "one sequence: no tab strip over the timeline");
    assert!(h.state().ui.timeline.auto_fit, "all tracks fitted");
    let d = harness(false);
    assert!(!d.state().tokens.hide_timeline_strip && !d.state().ui.timeline.auto_fit);
    assert!(d.state().ui.show_status_bar && d.state().tokens.tab_h >= 30.0, "default interface unchanged");
}
