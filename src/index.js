const { invoke } = window.__TAURI__.tauri;
const prints_={
    data() {
      return {
        prints:[["1","2","3"]],
        printmsg:"",
        url:"",
        printname:"",
        rulename:"",
        rulenamelist:["1","2","3"],
      }
    },
    methods: {
        get_prints() {
              invoke("get_prints", {}).then(res=>{
                this.prints=res;
              });
          },
        add_print(){
          invoke("add_print", {url:this.url,printname:this.printname,rulename:this.rulename}).then(res=>{
            this.printmsg=res;
          });
          this.get_prints()
        },
        del_print(){
          invoke("del_print", {url:this.url}).then(res=>{
            this.printmsg=res;
          });
          this.get_prints()
        },
        get_rule_list(){
          invoke("get_rule_list", {}).then(res=>{
            this.rulenamelist=res;
          });
        }
    },
    mounted() {
      this.get_prints()
      this.get_rule_list()
    }
  }
  Vue.createApp(prints_).mount('#prints')

  const rules_={
    data() {
      return {
        rules:[["1","2","3"]],
        rulemsg:"",
        rule:"",
        rulename:"",
      }
    },
    methods: {
        get_rules() {
              invoke("get_rules", {}).then(res=>{
                this.rules=res;
              });
          },
        add_rule(){
          invoke("add_rule", {rule:this.rule,rulename:this.rulename}).then(res=>{
            this.rulemsg=res;
          });
          this.get_rules()
        },
        del_rule(){
          invoke("del_rule", {rulename:this.rulename}).then(res=>{
            this.rulemsg=res;
          });
          this.get_rules()
        }
    },
    mounted() {
      this.get_rules()
    }
  }
  Vue.createApp(rules_).mount('#rules')

  const printdatas_={
    data() {
      return {
        datas:[["1","2","3"]],
      }
    },
    methods: {
        get_prints_data() {
              invoke("get_prints_data", {}).then(res=>{
                this.datas=res;
              });
          },
    },
    mounted() {
      this.get_prints_data()
    }
  }
  Vue.createApp(printdatas_).mount('#printsdata')
